# ductus proxy

HTTP CONNECT proxy with domain allowlist. Allowed domains tunnel through; others get 403.

## Run

```bash
cargo run -- [--port 8080] [--config config.toml] [--allowlist allowlist.txt]
```

### All Flags

| Flag | Default | Description |
|---|---|---|
| `--config <path>` | `config.toml` | TOML config file |
| `--port <port>` | `8080` | Listen port (`0` = auto-assign, see below) |
| `--allowlist <path>` | `allowlist.txt` | Permanent allowlist file |
| `--session-allowlist <path>` | — | Session-specific allowlist (combined with permanent) |
| `--blocked-log <path>` | — | Log blocked domains with timestamps |
| `--pidfile <path>` | — | Write PID to file (cleaned up on SIGTERM) |

CLI flags override config file values.

## Auto Port Assignment (`--port 0`)

When `--port 0` is passed, the OS assigns a free port. The actual port is printed to **stdout** (one line, just the number). All other output goes to stderr.

```bash
# Capture the assigned port
ductus --port 0 --allowlist allow.txt --pidfile /tmp/ductus.pid &
DUCTUS_PORT=$(head -1 <&3)  # or read from stdout pipe
export HTTP_PROXY="http://127.0.0.1:${DUCTUS_PORT}"

# Later: graceful stop
kill $(cat /tmp/ductus.pid)
```

This eliminates the need for shell-side port scanning (`ss` loops).

## Graceful Shutdown

ductus handles SIGTERM: stops accepting new connections and exits cleanly.

- Pidfile is automatically deleted on SIGTERM
- In-flight connections are dropped (not drained)
- SIGHUP continues to work for allowlist reload (separate from shutdown)

```bash
# Stop gracefully
kill $(cat /tmp/ductus.pid)
# or
kill -TERM <pid>
```

## Configuration

`config.toml`:
```toml
port = 8080
allowlist = "allowlist.txt"
```

## allowlist.txt

One domain per line. Comments with `#`. Wildcards with `*.`.

```
example.com
api.github.com
*.crates.io        # matches static.crates.io, index.crates.io, etc.
pypi.org           # trailing comments work too
```

Wildcard `*.example.com` matches subdomains (`api.example.com`) but not the root (`example.com`).

## Hot Reload (SIGHUP)

Send SIGHUP to reload both permanent and session allowlists without restart:

```bash
kill -HUP $(cat /tmp/ductus.pid)
```

## Behavior

| Condition | Response |
|---|---|
| Domain in allowlist | `200 Connection established` + TCP tunnel |
| Domain not in allowlist | `403 Forbidden` (see below) |
| Non-CONNECT method | `400 Bad Request` |
| Target unreachable | `502 Bad Gateway` |

## 403 Response Body

```
BLOCKED: <host>
ALLOWLIST: <allowlist_path>
```

The reader is an AI agent. It reads the 403, decides if access is needed, and appends the domain to the allowlist file.

## Usage with curl

```bash
curl --proxy http://localhost:8080 https://example.com   # allowed
curl --proxy http://localhost:8080 https://evil.com      # 403
```

## Usage with environment

```bash
export HTTPS_PROXY=http://localhost:8080
export HTTP_PROXY=http://localhost:8080
```
