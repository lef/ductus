use std::collections::HashSet;
use std::sync::{Arc, Mutex, RwLock};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

/// A domain allowlist supporting exact matches and wildcard patterns.
///
/// Wildcard entries use the `*.example.com` format and match any subdomain
/// of the specified domain (but not the domain itself).
pub struct Allowlist {
    exact: HashSet<String>,
    wildcards: Vec<String>,
}

impl Allowlist {
    /// Checks if the given domain is permitted by this allowlist.
    pub fn contains(&self, domain: &str) -> bool {
        self.exact.contains(domain) || self.wildcards.iter().any(|p| wildcard_match(p, domain))
    }
}

/// Matches a `*.suffix` pattern against a domain.
///
/// `*.github.com` matches `api.github.com` but not `github.com` itself,
/// and not `github.com.evil.com`.
fn wildcard_match(pattern: &str, domain: &str) -> bool {
    if let Some(suffix) = pattern.strip_prefix("*.") {
        domain.ends_with(suffix)
            && domain.len() > suffix.len()
            && domain.as_bytes()[domain.len() - suffix.len() - 1] == b'.'
    } else {
        pattern == domain
    }
}

/// Loads a domain allowlist from a file.
///
/// Lines starting with `#` are comments. Inline comments after `#` are stripped.
/// Entries starting with `*` are treated as wildcard patterns.
/// Returns an empty allowlist if the file does not exist.
pub fn load_allowlist(path: &str) -> Allowlist {
    let mut exact = HashSet::new();
    let mut wildcards = Vec::new();
    for entry in std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(|l| l.split('#').next().unwrap_or("").trim().to_owned())
        .filter(|l| !l.is_empty())
    {
        if entry.starts_with("*.") {
            wildcards.push(entry);
        } else {
            exact.insert(entry);
        }
    }
    Allowlist { exact, wildcards }
}

/// Reloads the allowlist from disk, replacing the current contents.
///
/// Intended for use with a SIGHUP handler. If the file cannot be read,
/// the allowlist becomes empty (same behavior as `load_allowlist`).
pub fn reload_allowlist(allowlist: &Arc<RwLock<Allowlist>>, path: &str) {
    let new = load_allowlist(path);
    if let Ok(mut guard) = allowlist.write() {
        *guard = new;
    }
}

/// Loads a merged allowlist from a permanent file and an optional session file.
///
/// Both files use the same format. Entries from both are combined into
/// a single `Allowlist`. If the session file does not exist or is `None`,
/// behaves identically to `load_allowlist`.
pub fn load_merged_allowlist(permanent: &str, session: Option<&str>) -> Allowlist {
    let mut al = load_allowlist(permanent);
    if let Some(session_path) = session {
        let session_al = load_allowlist(session_path);
        al.exact.extend(session_al.exact);
        al.wildcards.extend(session_al.wildcards);
    }
    al
}

/// Reloads the merged allowlist from disk (permanent + optional session).
pub fn reload_merged_allowlist(
    allowlist: &Arc<RwLock<Allowlist>>,
    permanent: &str,
    session: Option<&str>,
) {
    let new = load_merged_allowlist(permanent, session);
    if let Ok(mut guard) = allowlist.write() {
        *guard = new;
    }
}

/// Log of blocked domains with in-memory deduplication.
pub struct BlockedLog {
    seen: HashSet<String>,
    file: Option<std::fs::File>,
}

impl BlockedLog {
    /// Records a blocked domain. Deduplicates: only writes on first occurrence.
    pub fn record(&mut self, domain: &str) {
        if !self.seen.insert(domain.to_owned()) {
            return;
        }
        if let Some(ref mut f) = self.file {
            use std::io::Write;
            let ts = format_utc_now();
            let _ = writeln!(f, "{ts} {domain}");
        }
    }
}

/// Creates a new `BlockedLog`. If `path` is `None`, no file is written.
pub fn new_blocked_log(path: Option<&str>) -> Arc<Mutex<BlockedLog>> {
    let file = path.and_then(|p| {
        if let Some(parent) = std::path::Path::new(p).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::File::create(p).ok()
    });
    Arc::new(Mutex::new(BlockedLog {
        seen: HashSet::new(),
        file,
    }))
}

/// Formats the current time as `YYYY-MM-DDThh:mm:ssZ` without external crates.
fn format_utc_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Manual UTC conversion
    let days = secs / 86400;
    let time_of_day = secs % 86400;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;

    // Date calculation from days since epoch
    let mut y = 1970i64;
    let mut remaining = days as i64;
    loop {
        let days_in_year = if is_leap(y) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        y += 1;
    }
    let leap = is_leap(y);
    let month_days: [i64; 12] = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut m = 0;
    for md in &month_days {
        if remaining < *md {
            break;
        }
        remaining -= *md;
        m += 1;
    }
    format!(
        "{y:04}-{:02}-{:02}T{hours:02}:{minutes:02}:{seconds:02}Z",
        m + 1,
        remaining + 1
    )
}

fn is_leap(y: i64) -> bool {
    y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
}

/// Parses an HTTP CONNECT request line and returns the target `host:port`.
/// Returns `None` if the line is not a valid CONNECT request.
pub fn parse_connect_target(request_line: &str) -> Option<String> {
    let mut parts = request_line.split_whitespace();
    match parts.next() {
        Some("CONNECT") => parts.next().map(str::to_owned),
        _ => None,
    }
}

/// Runs the proxy accept loop on the given listener.
pub async fn run(
    listener: TcpListener,
    allowlist: Arc<RwLock<Allowlist>>,
    allowlist_path: Arc<String>,
    blocked_log: Arc<Mutex<BlockedLog>>,
) {
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle(
                    stream,
                    allowlist.clone(),
                    allowlist_path.clone(),
                    blocked_log.clone(),
                ));
            }
            Err(e) => {
                eprintln!("accept error: {e}");
            }
        }
    }
}

async fn handle(
    client: TcpStream,
    allowlist: Arc<RwLock<Allowlist>>,
    allowlist_path: Arc<String>,
    blocked_log: Arc<Mutex<BlockedLog>>,
) {
    if let Err(e) = handle_inner(client, allowlist, allowlist_path, blocked_log).await {
        eprintln!("handle error: {e}");
    }
}

async fn handle_inner(
    mut client: TcpStream,
    allowlist: Arc<RwLock<Allowlist>>,
    allowlist_path: Arc<String>,
    blocked_log: Arc<Mutex<BlockedLog>>,
) -> anyhow::Result<()> {
    let (reader, mut writer) = client.split();
    let mut lines = BufReader::new(reader).lines();
    let first = match lines.next_line().await? {
        Some(l) => l,
        None => return Ok(()),
    };
    // drain remaining headers until empty line
    while matches!(lines.next_line().await, Ok(Some(ref l)) if !l.is_empty()) {}

    let host = match parse_connect_target(&first) {
        Some(h) => h,
        None => {
            let _ = writer.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n").await;
            return Ok(());
        }
    };
    let domain = host.split(':').next().unwrap_or(&host);

    // Acquire the read lock and drop it before any .await
    let blocked = {
        let guard = allowlist
            .read()
            .map_err(|e| anyhow::anyhow!("allowlist lock poisoned: {e}"))?;
        !guard.contains(domain)
    };

    if blocked {
        // Record blocked domain (lock is std::sync::Mutex, no .await while held)
        if let Ok(mut log) = blocked_log.lock() {
            log.record(domain);
        }
        let body = format!("BLOCKED: {domain}\nALLOWLIST: {allowlist_path}\n");
        let response = format!(
            "HTTP/1.1 403 Forbidden\r\nContent-Length: {}\r\n\r\n{body}",
            body.len()
        );
        let _ = writer.write_all(response.as_bytes()).await;
        return Ok(());
    }

    let mut target = match TcpStream::connect(&host).await {
        Ok(s) => s,
        Err(_) => {
            let _ = writer.write_all(b"HTTP/1.1 502 Bad Gateway\r\n\r\n").await;
            return Ok(());
        }
    };
    let _ = writer
        .write_all(b"HTTP/1.1 200 Connection established\r\n\r\n")
        .await;
    let _ = io::copy_bidirectional(&mut client, &mut target).await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_temp(content: &str) -> tempfile::NamedTempFile {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f.flush().unwrap();
        f
    }

    #[test]
    fn load_allowlist_parses_comments_and_hosts() {
        let tmp = write_temp(
            "# header comment\n\
             example.com:443\n\
             api.example.com\n",
        );
        let al = load_allowlist(tmp.path().to_str().unwrap());
        assert!(al.contains("example.com:443"));
        assert!(al.contains("api.example.com"));
        assert!(!al.contains("other.com"));
    }

    #[test]
    fn load_allowlist_skips_empty_lines() {
        let tmp = write_temp("\n\nexample.com\n\n");
        let al = load_allowlist(tmp.path().to_str().unwrap());
        assert!(al.contains("example.com"));
        assert!(!al.contains("other.com"));
    }

    #[test]
    fn load_allowlist_skips_comment_only_lines() {
        let tmp = write_temp("# just a comment\n# another\n");
        let al = load_allowlist(tmp.path().to_str().unwrap());
        assert!(!al.contains("anything"));
    }

    #[test]
    fn load_allowlist_strips_trailing_comment() {
        let tmp = write_temp("example.com:443  # trailing comment\n");
        let al = load_allowlist(tmp.path().to_str().unwrap());
        assert!(al.contains("example.com:443"));
    }

    #[test]
    fn load_allowlist_missing_file_returns_empty() {
        let al = load_allowlist("/nonexistent/path/does-not-exist.txt");
        assert!(!al.contains("anything"));
    }

    // --- Wildcard allowlist tests ---

    fn allowlist_from_str(content: &str) -> Allowlist {
        let tmp = write_temp(content);
        load_allowlist(tmp.path().to_str().unwrap())
    }

    #[test]
    fn wildcard_matches_subdomain() {
        let al = allowlist_from_str("*.github.com\n");
        assert!(al.contains("api.github.com"));
    }

    #[test]
    fn wildcard_does_not_match_root() {
        let al = allowlist_from_str("*.github.com\n");
        assert!(!al.contains("github.com"));
    }

    #[test]
    fn wildcard_does_not_match_other_domain() {
        let al = allowlist_from_str("*.github.com\n");
        assert!(!al.contains("api.evil.com"));
    }

    #[test]
    fn wildcard_no_bypass_with_suffix_injection() {
        // must not match when the pattern suffix appears inside a different domain
        let al = allowlist_from_str("*.github.com\n");
        assert!(!al.contains("github.com.evil.com"));
    }

    #[test]
    fn exact_and_wildcard_coexist() {
        let al = allowlist_from_str("example.com\n*.github.com\n");
        assert!(al.contains("example.com"));
        assert!(al.contains("api.github.com"));
        assert!(!al.contains("evil.com"));
    }

    #[test]
    fn load_allowlist_parses_wildcards() {
        let tmp = write_temp("*.github.com\nexample.com\n");
        let al = load_allowlist(tmp.path().to_str().unwrap());
        assert!(al.contains("api.github.com"));
        assert!(al.contains("example.com"));
        assert!(!al.contains("github.com"));
    }

    // --- reload_allowlist tests ---

    #[test]
    fn reload_adds_new_domain() {
        let tmp = write_temp("example.com\n");
        let path = tmp.path().to_str().unwrap().to_string();
        let al = Arc::new(std::sync::RwLock::new(load_allowlist(&path)));
        assert!(al.read().unwrap().contains("example.com"));
        assert!(!al.read().unwrap().contains("new-domain.com"));

        // Simulate editing the file
        std::fs::write(tmp.path(), "example.com\nnew-domain.com\n").unwrap();
        reload_allowlist(&al, &path);

        assert!(al.read().unwrap().contains("new-domain.com"));
    }

    #[test]
    fn reload_handles_missing_file() {
        let al = Arc::new(std::sync::RwLock::new(load_allowlist("/nonexistent")));
        // Should not panic
        reload_allowlist(&al, "/nonexistent");
        assert!(!al.read().unwrap().contains("anything"));
    }

    // --- parse_connect_target tests ---

    #[test]
    fn parse_connect_target_valid_request() {
        let result = parse_connect_target("CONNECT example.com:443 HTTP/1.1");
        assert_eq!(result, Some("example.com:443".to_owned()));
    }

    #[test]
    fn parse_connect_target_non_connect_method() {
        let result = parse_connect_target("GET / HTTP/1.1");
        assert_eq!(result, None);
    }

    #[test]
    fn parse_connect_target_empty_line() {
        let result = parse_connect_target("");
        assert_eq!(result, None);
    }

    #[test]
    fn parse_connect_target_missing_target() {
        let result = parse_connect_target("CONNECT");
        assert_eq!(result, None);
    }

    // --- BlockedLog tests ---

    #[test]
    fn blocked_log_no_path_no_file() {
        let log = new_blocked_log(None);
        // Should not panic
        log.lock().unwrap().record("example.com");
    }

    #[test]
    fn blocked_log_records_to_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("blocked.log");
        let path_str = path.to_str().unwrap();
        let log = new_blocked_log(Some(path_str));
        log.lock().unwrap().record("evil.com");
        drop(log); // flush
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("evil.com"), "content: {content}");
    }

    #[test]
    fn blocked_log_deduplicates() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("blocked.log");
        let path_str = path.to_str().unwrap();
        let log = new_blocked_log(Some(path_str));
        {
            let mut guard = log.lock().unwrap();
            guard.record("evil.com");
            guard.record("evil.com");
        }
        drop(log);
        let content = std::fs::read_to_string(&path).unwrap();
        let lines: Vec<_> = content.lines().filter(|l| !l.is_empty()).collect();
        assert_eq!(lines.len(), 1, "expected 1 line, got: {lines:?}");
    }

    #[test]
    fn blocked_log_timestamp_format() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("blocked.log");
        let path_str = path.to_str().unwrap();
        let log = new_blocked_log(Some(path_str));
        log.lock().unwrap().record("evil.com");
        drop(log);
        let content = std::fs::read_to_string(&path).unwrap();
        let line = content.lines().next().unwrap();
        // Format: 2026-03-13T14:23:01Z evil.com
        assert!(line.len() > 20, "line too short: {line}");
        let ts = &line[..20];
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], "T");
        assert_eq!(&ts[13..14], ":");
        assert_eq!(&ts[16..17], ":");
        assert_eq!(&ts[19..20], "Z");
    }

    #[test]
    fn blocked_log_multiple_domains() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("blocked.log");
        let path_str = path.to_str().unwrap();
        let log = new_blocked_log(Some(path_str));
        {
            let mut guard = log.lock().unwrap();
            guard.record("a.com");
            guard.record("b.com");
            guard.record("a.com"); // duplicate
        }
        drop(log);
        let content = std::fs::read_to_string(&path).unwrap();
        let lines: Vec<_> = content.lines().filter(|l| !l.is_empty()).collect();
        assert_eq!(lines.len(), 2, "expected 2 lines, got: {lines:?}");
    }

    #[test]
    fn blocked_log_creates_parent_dir() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("dir").join("blocked.log");
        let path_str = path.to_str().unwrap();
        let log = new_blocked_log(Some(path_str));
        log.lock().unwrap().record("evil.com");
        drop(log);
        assert!(path.exists(), "file should exist at nested path");
    }

    // --- load_merged_allowlist tests ---

    #[test]
    fn load_merged_no_session() {
        let perm = write_temp("example.com\n");
        let perm_path = perm.path().to_str().unwrap();
        let al = load_merged_allowlist(perm_path, None);
        assert!(al.contains("example.com"));
        assert!(!al.contains("other.com"));
    }

    #[test]
    fn load_merged_adds_session_entries() {
        let perm = write_temp("a.com\n");
        let session = write_temp("b.com\n");
        let al = load_merged_allowlist(
            perm.path().to_str().unwrap(),
            Some(session.path().to_str().unwrap()),
        );
        assert!(al.contains("a.com"));
        assert!(al.contains("b.com"));
    }

    #[test]
    fn load_merged_session_missing_ok() {
        let perm = write_temp("a.com\n");
        let al = load_merged_allowlist(
            perm.path().to_str().unwrap(),
            Some("/nonexistent/session.txt"),
        );
        assert!(al.contains("a.com"));
        // No panic, no error
    }

    #[test]
    fn load_merged_session_wildcards() {
        let perm = write_temp("example.com\n");
        let session = write_temp("*.crates.io\n");
        let al = load_merged_allowlist(
            perm.path().to_str().unwrap(),
            Some(session.path().to_str().unwrap()),
        );
        assert!(al.contains("static.crates.io"));
        assert!(!al.contains("crates.io")); // wildcard doesn't match root
    }

    #[test]
    fn reload_merged_picks_up_session_change() {
        let perm = write_temp("a.com\n");
        let session = write_temp("b.com\n");
        let perm_path = perm.path().to_str().unwrap().to_string();
        let session_path = session.path().to_str().unwrap().to_string();

        let al = Arc::new(RwLock::new(load_merged_allowlist(
            &perm_path,
            Some(&session_path),
        )));
        assert!(!al.read().unwrap().contains("c.com"));

        // Append to session file
        std::fs::write(session.path(), "b.com\nc.com\n").unwrap();
        reload_merged_allowlist(&al, &perm_path, Some(&session_path));

        assert!(al.read().unwrap().contains("c.com"));
    }

    #[test]
    fn reload_merged_session_none_ok() {
        let perm = write_temp("a.com\n");
        let perm_path = perm.path().to_str().unwrap().to_string();
        let al = Arc::new(RwLock::new(load_merged_allowlist(&perm_path, None)));
        // Should not panic
        reload_merged_allowlist(&al, &perm_path, None);
        assert!(al.read().unwrap().contains("a.com"));
    }

    // --- format_utc_now test ---

    #[test]
    fn format_utc_now_has_correct_shape() {
        let ts = format_utc_now();
        assert_eq!(ts.len(), 20, "timestamp should be 20 chars: {ts}");
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], "T");
        assert_eq!(&ts[19..20], "Z");
    }
}
