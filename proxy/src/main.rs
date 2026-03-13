use clap::Parser;
use serde::Deserialize;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "config.toml")]
    config: String,
    #[arg(long)]
    port: Option<u16>,
    #[arg(long)]
    allowlist: Option<String>,
}

#[derive(Deserialize, Default)]
struct Config {
    port: Option<u16>,
    allowlist: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let cfg: Config = std::fs::read_to_string(&args.config)
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default();
    let port = args.port.or(cfg.port).unwrap_or(8080);
    let allowlist_path = Arc::new(
        args.allowlist
            .or(cfg.allowlist)
            .unwrap_or_else(|| "allowlist.txt".into()),
    );
    let allowlist = Arc::new(RwLock::new(ductus::load_allowlist(&allowlist_path)));
    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .map_err(|e| anyhow::anyhow!("failed to bind to port {port}: {e}"))?;
    eprintln!(":: ductus listening on :{port}");

    // Reload allowlist on SIGHUP
    let al_sig = allowlist.clone();
    let path_sig = allowlist_path.clone();
    tokio::spawn(async move {
        let mut sig = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::hangup())
            .expect("failed to install SIGHUP handler");
        loop {
            sig.recv().await;
            eprintln!(":: SIGHUP — reloading allowlist from {path_sig}");
            ductus::reload_allowlist(&al_sig, &path_sig);
            eprintln!(":: allowlist reloaded");
        }
    });

    ductus::run(listener, allowlist, allowlist_path).await;
    Ok(())
}
