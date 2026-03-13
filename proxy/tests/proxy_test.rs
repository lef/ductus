use std::io::Write;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

/// Spawn the proxy accept loop on a random port.
/// Returns the assigned port.
async fn spawn_proxy(allowed: &[&str]) -> u16 {
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    for host in allowed {
        writeln!(tmp, "{host}").unwrap();
    }
    tmp.flush().unwrap();
    let allowlist_path = tmp.path().to_str().unwrap().to_string();
    let allowlist = Arc::new(ductus::load_allowlist(&allowlist_path));
    let allowlist_path = Arc::new(allowlist_path);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    // Leak the tempfile handle to keep it alive for the test
    let _keep = Box::leak(Box::new(tmp));
    tokio::spawn(async move {
        ductus::run(listener, allowlist, allowlist_path).await;
    });
    port
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
