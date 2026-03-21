# Feature Flags

## When to Use Feature Flags

YAGNI: **デフォルトはフラグなし**。以下の場合のみ追加する:

| Use feature flag | Don't use |
|---|---|
| Optional heavy dependency (e.g., TLS backend) | Internal implementation switches |
| Platform-specific code (e.g., Windows-only) | "Just in case" optional behavior |
| Optional binary in the same crate | dev-only functionality → use `dev-dependencies` |
| Compatibility shim for older API | |

ライブラリで feature flag が多すぎると、利用者の混乱とCI負担が増える。

## Naming Conventions

```toml
[features]
# ✓ 機能を表す名詞
tls = ["dep:rustls"]
sqlite = ["dep:rusqlite"]
cli = ["dep:clap"]

# ✓ バックエンドの選択
rustls-tls = ["dep:rustls"]
native-tls = ["dep:native-tls"]

# ✗ 避ける: 実装の詳細
use-new-parser = []
experimental = []
```

## Default Features: Keep Minimal

```toml
[features]
default = []           # 依存関係ゼロが理想

# 必要なときだけ default に追加
default = ["tls"]     # TLS なしでは使いものにならないライブラリの場合のみ
```

`default-features = false` を強制させるのはライブラリ利用者の負担になる。
default を太くするより、利用者が明示的に選べる設計にする。

## Declaring Optional Dependencies

```toml
[dependencies]
# dep: prefix で optional dep を明示 (Rust 1.60+)
rustls = { version = "0.23", optional = true }
serde = { version = "1", optional = true, features = ["derive"] }

[features]
tls = ["dep:rustls"]
serde = ["dep:serde"]
```

## Feature-Gated Code

```rust
#[cfg(feature = "tls")]
pub mod tls {
    pub fn wrap(stream: TcpStream) -> TlsStream { ... }
}

// 条件付きインポート
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Config { ... }
```

## Testing Feature Combinations

```bash
# デフォルト（features なし）
cargo test

# 特定 feature
cargo test --features tls

# 全 features
cargo test --all-features

# features なし（default も無効化）
cargo test --no-default-features
```

CI では最低 `--no-default-features` と `--all-features` の両方を走らせる。

Feature が増えると `2^n` の組み合わせ爆発が起きる。
フラグが 4 個を超えたら設計を見直す。

## Feature Flag Explosion の回避

```
フラグが増えてきたら:
→ 別クレートに切り出す（workspace 利用）
→ runtime 設定に移す（enum や設定ファイル）
→ 本当に必要か YAGNI で再評価する
```
