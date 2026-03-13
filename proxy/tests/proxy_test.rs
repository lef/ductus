use std::io::Write;
use std::sync::{Arc, RwLock};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

/// Spawn the proxy accept loop on a random port.
/// Returns the assigned port.
async fn spawn_proxy(allowed: &[&str]) -> u16 {
    spawn_proxy_with_opts(allowed, None, None).await.0
}

struct ProxyHandle {
    _perm_file: Box<tempfile::NamedTempFile>,
    _session_file: Option<Box<tempfile::NamedTempFile>>,
}

/// Spawn the proxy with optional session allowlist and blocked log.
/// Returns (port, ProxyHandle).
async fn spawn_proxy_with_opts(
    allowed: &[&str],
    session_allowed: Option<&[&str]>,
    blocked_log_path: Option<&str>,
) -> (u16, ProxyHandle) {
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    for host in allowed {
        writeln!(tmp, "{host}").unwrap();
    }
    tmp.flush().unwrap();
    let perm_path = tmp.path().to_str().unwrap().to_string();

    let session_file = session_allowed.map(|hosts| {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        for host in hosts {
            writeln!(f, "{host}").unwrap();
        }
        f.flush().unwrap();
        Box::new(f)
    });
    let session_path = session_file
        .as_ref()
        .map(|f| f.path().to_str().unwrap().to_string());

    let allowlist = Arc::new(RwLock::new(ductus::load_merged_allowlist(
        &perm_path,
        session_path.as_deref(),
    )));
    let allowlist_path = Arc::new(perm_path);
    let blocked_log = ductus::new_blocked_log(blocked_log_path);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let perm_file = Box::new(tmp);
    tokio::spawn(async move {
        ductus::run(listener, allowlist, allowlist_path, blocked_log).await;
    });
    (
        port,
        ProxyHandle {
            _perm_file: perm_file,
            _session_file: session_file,
        },
    )
}

/// Spawn a simple echo server on a random port.
/// Accepts one connection, echoes back whatever it receives.
async fn spawn_echo_server() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        if let Ok((mut stream, _)) = listener.accept().await {
            let (mut reader, mut writer) = stream.split();
            let _ = tokio::io::copy(&mut reader, &mut writer).await;
        }
    });
    port
}

/// Send a raw request line to the proxy and return the first response line.
async fn send_connect(proxy_port: u16, request_line: &str) -> String {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{proxy_port}"))
        .await
        .unwrap();
    let request = format!("{request_line}\r\n\r\n");
    stream.write_all(request.as_bytes()).await.unwrap();

    let mut reader = BufReader::new(stream);
    let mut first_line = String::new();
    reader.read_line(&mut first_line).await.unwrap();
    first_line
}

#[tokio::test]
async fn allowed_domain_gets_200() {
    let echo_port = spawn_echo_server().await;
    let proxy_port = spawn_proxy(&["127.0.0.1"]).await;

    let response = send_connect(
        proxy_port,
        &format!("CONNECT 127.0.0.1:{echo_port} HTTP/1.1"),
    )
    .await;
    assert!(
        response.starts_with("HTTP/1.1 200"),
        "expected 200, got: {response}"
    );
}

#[tokio::test]
async fn blocked_domain_gets_403() {
    let proxy_port = spawn_proxy(&[]).await; // empty allowlist

    let response = send_connect(proxy_port, "CONNECT blocked.example.com:443 HTTP/1.1").await;
    assert!(
        response.starts_with("HTTP/1.1 403"),
        "expected 403, got: {response}"
    );
}

#[tokio::test]
async fn non_connect_method_gets_400() {
    let proxy_port = spawn_proxy(&[]).await;

    let response = send_connect(proxy_port, "GET / HTTP/1.1").await;
    assert!(
        response.starts_with("HTTP/1.1 400"),
        "expected 400, got: {response}"
    );
}

#[tokio::test]
async fn unreachable_target_gets_502() {
    // Port 1 should be unreachable on localhost
    let proxy_port = spawn_proxy(&["127.0.0.1"]).await;

    let response = send_connect(proxy_port, "CONNECT 127.0.0.1:1 HTTP/1.1").await;
    assert!(
        response.starts_with("HTTP/1.1 502"),
        "expected 502, got: {response}"
    );
}

// --- Session allowlist integration tests ---

#[tokio::test]
async fn session_allowlist_domain_gets_200() {
    let echo_port = spawn_echo_server().await;
    // 127.0.0.1 only in session allowlist, not permanent
    let (proxy_port, _handle) = spawn_proxy_with_opts(&[], Some(&["127.0.0.1"]), None).await;

    let response = send_connect(
        proxy_port,
        &format!("CONNECT 127.0.0.1:{echo_port} HTTP/1.1"),
    )
    .await;
    assert!(
        response.starts_with("HTTP/1.1 200"),
        "expected 200, got: {response}"
    );
}

#[tokio::test]
async fn blocked_domain_not_in_either_gets_403() {
    let (proxy_port, _handle) =
        spawn_proxy_with_opts(&["good.com"], Some(&["session.com"]), None).await;

    let response = send_connect(proxy_port, "CONNECT evil.com:443 HTTP/1.1").await;
    assert!(
        response.starts_with("HTTP/1.1 403"),
        "expected 403, got: {response}"
    );
}

#[tokio::test]
async fn blocked_domain_written_to_log() {
    let dir = tempfile::tempdir().unwrap();
    let log_path = dir.path().join("blocked.log");
    let log_path_str = log_path.to_str().unwrap().to_string();

    let (proxy_port, _handle) = spawn_proxy_with_opts(&[], None, Some(&log_path_str)).await;

    let _ = send_connect(proxy_port, "CONNECT evil.com:443 HTTP/1.1").await;
    // Give a moment for the log to be written
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let content = std::fs::read_to_string(&log_path).unwrap();
    assert!(
        content.contains("evil.com"),
        "blocked log should contain evil.com: {content}"
    );
}

#[tokio::test]
async fn blocked_domain_logged_once() {
    let dir = tempfile::tempdir().unwrap();
    let log_path = dir.path().join("blocked.log");
    let log_path_str = log_path.to_str().unwrap().to_string();

    let (proxy_port, _handle) = spawn_proxy_with_opts(&[], None, Some(&log_path_str)).await;

    // Send twice
    let _ = send_connect(proxy_port, "CONNECT evil.com:443 HTTP/1.1").await;
    let _ = send_connect(proxy_port, "CONNECT evil.com:443 HTTP/1.1").await;
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let content = std::fs::read_to_string(&log_path).unwrap();
    let lines: Vec<_> = content.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines.len(),
        1,
        "expected 1 line (dedup), got {}: {lines:?}",
        lines.len()
    );
}

#[tokio::test]
async fn no_blocked_log_when_not_specified() {
    let dir = tempfile::tempdir().unwrap();
    let log_path = dir.path().join("should-not-exist.log");

    let proxy_port = spawn_proxy(&[]).await;

    let _ = send_connect(proxy_port, "CONNECT evil.com:443 HTTP/1.1").await;
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    assert!(
        !log_path.exists(),
        "log file should not exist when --blocked-log is not specified"
    );
}
