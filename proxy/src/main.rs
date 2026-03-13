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
    #[arg(long)]
    session_allowlist: Option<String>,
    #[arg(long)]
    blocked_log: Option<String>,
    #[arg(long)]
    pidfile: Option<String>,
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
    let allowlist = Arc::new(RwLock::new(ductus::load_merged_allowlist(
        &allowlist_path,
        args.session_allowlist.as_deref(),
    )));
    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .map_err(|e| anyhow::anyhow!("failed to bind to port {port}: {e}"))?;
    eprintln!(":: ductus listening on :{port}");

    let blocked_log = ductus::new_blocked_log(args.blocked_log.as_deref());

    // Write pidfile if requested
    if let Some(ref pidfile) = args.pidfile {
        let pid = std::process::id();
        std::fs::write(pidfile, pid.to_string())
            .map_err(|e| anyhow::anyhow!("failed to write pidfile {pidfile}: {e}"))?;
    }

    // Reload allowlist on SIGHUP
    let al_sig = allowlist.clone();
    let perm_path_sig = allowlist_path.clone();
    let session_path_sig = args.session_allowlist.map(Arc::new);
    let session_sig = session_path_sig.clone();
    tokio::spawn(async move {
        let mut sig = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::hangup())
            .expect("failed to install SIGHUP handler");
        loop {
            sig.recv().await;
            eprintln!(":: SIGHUP — reloading allowlist");
            ductus::reload_merged_allowlist(
                &al_sig,
                &perm_path_sig,
                session_sig.as_ref().map(|s| s.as_str()),
            );
            eprintln!(":: allowlist reloaded");
        }
    });

    ductus::run(listener, allowlist, allowlist_path, blocked_log).await;
    Ok(())
}
