# Fuzzing

## When Fuzzing Is Worth It

| Worth fuzzing | Not worth fuzzing |
|---|---|
| Parsers (text, binary, network protocols) | Pure business logic |
| Anything handling untrusted external input | Unit-tested pure functions |
| Deserialization (serde, custom formats) | UI / presentation code |
| Cryptographic input processing | Trivial wrappers |
| File format readers | Already well-covered by property tests |

**ファジングはユニットテストの代替ではない**。
ユニットテスト → プロパティテスト → ファジングの順に導入する。

## Setup with cargo-fuzz

```bash
# インストール（nightly 必須）
cargo install cargo-fuzz

# プロジェクトに fuzz ターゲットを追加
cargo fuzz init
cargo fuzz add fuzz_target_1
```

生成されるファイル:

```
fuzz/
├── Cargo.toml
└── fuzz_targets/
    └── fuzz_target_1.rs
```

## Writing a Fuzz Target

```rust
// fuzz/fuzz_targets/fuzz_target_1.rs
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // パニックしてはいけない（意図的な panic! は OK）
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = my_crate::parse_host(s);
    }
});
```

構造化入力を使う場合:

```rust
use libfuzzer_sys::fuzz_target;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    host: String,
    port: u16,
}

fuzz_target!(|input: FuzzInput| {
    let _ = my_crate::connect(&input.host, input.port);
});
```

```toml
# fuzz/Cargo.toml に追加
[dependencies]
arbitrary = { version = "1", features = ["derive"] }
```

## Running

```bash
# ファジング実行（Ctrl+C で停止）
cargo fuzz run fuzz_target_1

# コーパスディレクトリを指定
cargo fuzz run fuzz_target_1 fuzz/corpus/fuzz_target_1

# クラッシュを再現
cargo fuzz run fuzz_target_1 fuzz/artifacts/fuzz_target_1/crash-<hash>

# カバレッジレポート
cargo fuzz coverage fuzz_target_1
```

## Corpus Management

```bash
fuzz/
└── corpus/
    └── fuzz_target_1/
        ├── seed1          ← 手動で用意した代表的な入力
        └── seed2
```

**コーパスをバージョン管理に含める**。
cargo-fuzz が発見した興味深いケースは `corpus/` に追加する。
クラッシュ再現ファイルも `artifacts/` として保存する。

```bash
# コーパスを最小化（冗長な入力を除去）
cargo fuzz cmin fuzz_target_1
```

## Integration with CI

```yaml
# ファジングは CI で短時間だけ走らせる（リグレッション検出用）
- name: Fuzz (short run)
  run: |
    cargo fuzz run fuzz_target_1 -- -max_total_time=60
```

長時間のファジングはローカルまたは専用のファジングインフラで行う。
OSS-Fuzz への統合も検討（オープンソースプロジェクトの場合）。
