use std::collections::HashSet;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

/// Loads a domain allowlist from a file.
/// Lines starting with `#` are comments. Inline comments after `#` are stripped.
/// Returns an empty set if the file does not exist.
pub fn load_allowlist(path: &str) -> HashSet<String> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(|l| l.split('#').next().unwrap_or("").trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect()
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
    allowlist: Arc<HashSet<String>>,
    allowlist_path: Arc<String>,
) {
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

async fn handle(client: TcpStream, allowlist: Arc<HashSet<String>>, allowlist_path: Arc<String>) {
    if let Err(e) = handle_inner(client, allowlist, allowlist_path).await {
        eprintln!("handle error: {e}");
    }
}

async fn handle_inner(
    mut client: TcpStream,
    allowlist: Arc<HashSet<String>>,
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
        let set = load_allowlist(tmp.path().to_str().unwrap());
        assert_eq!(set.len(), 2);
        assert!(set.contains("example.com:443"));
        assert!(set.contains("api.example.com"));
    }

    #[test]
    fn load_allowlist_skips_empty_lines() {
        let tmp = write_temp("\n\nexample.com\n\n");
        let set = load_allowlist(tmp.path().to_str().unwrap());
        assert_eq!(set.len(), 1);
        assert!(set.contains("example.com"));
    }

    #[test]
    fn load_allowlist_skips_comment_only_lines() {
        let tmp = write_temp("# just a comment\n# another\n");
        let set = load_allowlist(tmp.path().to_str().unwrap());
        assert!(set.is_empty());
    }

    #[test]
    fn load_allowlist_strips_trailing_comment() {
        let tmp = write_temp("example.com:443  # trailing comment\n");
        let set = load_allowlist(tmp.path().to_str().unwrap());
        assert_eq!(set.len(), 1);
        assert!(set.contains("example.com:443"));
    }

    #[test]
    fn load_allowlist_missing_file_returns_empty() {
        let set = load_allowlist("/nonexistent/path/does-not-exist.txt");
        assert!(set.is_empty());
    }

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
