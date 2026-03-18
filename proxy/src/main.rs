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
    #[arg(long, conflicts_with = "no_pidfile")]
    pidfile: Option<String>,
    #[arg(long, conflicts_with = "pidfile")]
    no_pidfile: bool,
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
    let actual_port = listener.local_addr()?.port();
    if port == 0 {
        println!("{actual_port}");
    }
    eprintln!(":: ductus listening on :{actual_port}");

    let blocked_log = ductus::new_blocked_log(args.blocked_log.as_deref());

    // Write pidfile (default: /tmp/ductus.pid, disable with --no-pidfile)
    let effective_pidfile = if args.no_pidfile {
        None
    } else {
        Some(
            args.pidfile
                .unwrap_or_else(|| "/tmp/ductus.pid".to_string()),
        )
    };
    if let Some(ref pidfile) = effective_pidfile {
        let pid = std::process::id();
        std::fs::write(pidfile, pid.to_string())
            .map_err(|e| anyhow::anyhow!("failed to write pidfile {pidfile}: {e}"))?;
    }

    // Reload allowlist on SIGHUP — install handler eagerly (before accept loop)
    // to prevent default SIGHUP action (terminate) from killing the process
    let al_sig = allowlist.clone();
    let perm_path_sig = allowlist_path.clone();
    let session_path_sig = args.session_allowlist.map(Arc::new);
    let session_sig = session_path_sig.clone();
    match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::hangup()) {
        Ok(mut sig) => {
            tokio::spawn(async move {
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
        }
        Err(e) => {
            eprintln!(":: warning: failed to install SIGHUP handler: {e}");
        }
    }

    // Graceful shutdown on SIGTERM
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .map_err(|e| anyhow::anyhow!("failed to install SIGTERM handler: {e}"))?;
    ductus::run(
        listener,
        allowlist,
        allowlist_path,
        blocked_log,
        async move {
            sigterm.recv().await;
            eprintln!(":: SIGTERM received — shutting down");
        },
    )
    .await;

    // Clean up pidfile
    if let Some(ref pidfile) = effective_pidfile {
        let _ = std::fs::remove_file(pidfile);
    }
    Ok(())
}
