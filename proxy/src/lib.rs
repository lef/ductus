use std::collections::HashSet;
use std::sync::Arc;
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
pub async fn run(listener: TcpListener, allowlist: Arc<Allowlist>, allowlist_path: Arc<String>) {
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(handle(stream, allowlist.clone(), allowlist_path.clone()));
            }
            Err(e) => {
                eprintln!("accept error: {e}");
            }
        }
    }
}

async fn handle(client: TcpStream, allowlist: Arc<Allowlist>, allowlist_path: Arc<String>) {
    if let Err(e) = handle_inner(client, allowlist, allowlist_path).await {
        eprintln!("handle error: {e}");
    }
}

async fn handle_inner(
    mut client: TcpStream,
    allowlist: Arc<Allowlist>,
    allowlist_path: Arc<String>,
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

    if !allowlist.contains(domain) {
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
}
