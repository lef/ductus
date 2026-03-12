# ductus proxy

HTTP CONNECT proxy with domain allowlist. Allowed domains tunnel through; others get 403.

## Run

```bash
cargo run -- [--port 8080] [--config config.toml] [--allowlist allowlist.txt]
```

## Configuration

`config.toml`:
```toml
port = 8080
allowlist = "allowlist.txt"
```

CLI flags override config file values.

## allowlist.txt

One domain per line. Comments with `#`.

```
example.com
api.github.com
# trailing comments work too
pypi.org  # Python packages
```

## Behavior

| Condition | Response |
|---|---|
| Domain in allowlist | `200 Connection established` + TCP tunnel |
| Domain not in allowlist | `403 Forbidden` (see below) |

## 403 Response Body

```
BLOCKED: <host>
ALLOWLIST: <allowlist_path>
```

Example:
```
BLOCKED: evil.com
ALLOWLIST: /home/user/allowlist.txt
```

To allow the domain, add `<host>` as a new line to the file at `<allowlist_path>`.

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
