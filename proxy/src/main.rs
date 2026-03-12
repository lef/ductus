use clap::Parser;
use serde::Deserialize;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "config.toml")] config: String,
    #[arg(long)] port: Option<u16>,
    #[arg(long)] allowlist: Option<String>,
}

#[derive(Deserialize, Default)]
struct Config {
    port: Option<u16>,
    allowlist: Option<String>,
}

fn load_allowlist(path: &str) -> HashSet<String> {
    std::fs::read_to_string(path).unwrap_or_default()
        .lines()
        .map(|l| l.split('#').next().unwrap_or("").trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect()
}

async fn handle(mut client: TcpStream, allowlist: Arc<HashSet<String>>, allowlist_path: Arc<String>) {
    let (reader, mut writer) = client.split();
    let mut lines = BufReader::new(reader).lines();
    let first = match lines.next_line().await { Ok(Some(l)) => l, _ => return };
    // drain headers
    while matches!(lines.next_line().await, Ok(Some(l)) if !l.is_empty()) {}

    let host = match first.split_whitespace().nth(1) { Some(h) => h.to_owned(), None => return };
    let domain = host.split(':').next().unwrap_or(&host);

    if !allowlist.contains(domain) {
        let body = format!(
            "BLOCKED: {} is not in the allowlist.\nTo allow this domain, add it to {}:\n  echo \"{}\" >> {}\n",
            domain, allowlist_path, domain, allowlist_path
        );
        let _ = writer.write_all(
            format!("HTTP/1.1 403 Forbidden\r\nContent-Length: {}\r\n\r\n{}", body.len(), body).as_bytes()
        ).await;
        return;
    }

    let mut target = match TcpStream::connect(&host).await { Ok(s) => s, Err(_) => return };
    let _ = writer.write_all(b"HTTP/1.1 200 Connection established\r\n\r\n").await;
    let _ = io::copy_bidirectional(&mut client, &mut target).await;
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let cfg: Config = std::fs::read_to_string(&args.config)
        .ok().and_then(|s| toml::from_str(&s).ok()).unwrap_or_default();
    let port = args.port.or(cfg.port).unwrap_or(8080);
    let allowlist_path = Arc::new(args.allowlist.or(cfg.allowlist).unwrap_or_else(|| "allowlist.txt".into()));
    let allowlist = Arc::new(load_allowlist(&allowlist_path));
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    eprintln!(":: ductus listening on :{port}");
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(handle(stream, allowlist.clone(), allowlist_path.clone()));
    }
}
