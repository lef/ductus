# Profiling and Performance

## Core Principle

> 測定なしに最適化しない。

AI 生成コードは**正しいが遅い**ことがある。特に:
- `clone()` の多用（借用で済む場所）
- 不必要な `Vec` アロケーション
- O(n²) アルゴリズム（LLMは明示しない限り最適化しない）
- ホットパスでの `String` フォーマット

## Profiling Tools

```bash
# flamegraph — CPU ホットスポットの視覚化（最初の一手）
cargo install flamegraph
cargo flamegraph --bin my-binary -- [args]
# ブラウザで flamegraph.svg を開く

# perf (Linux)
cargo build --release
perf record -g ./target/release/my-binary
perf report

# heaptrack — メモリアロケーション分析
heaptrack ./target/release/my-binary
heaptrack_gui heaptrack.*.gz
```

**必ず `--release` ビルドで測定する**。デバッグビルドは 10-100x 遅い。

## Benchmarking with criterion

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false
```

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_parse(c: &mut Criterion) {
    c.bench_function("parse_host", |b| {
        b.iter(|| parse_host(black_box("example.com:443")))
    });
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
```

```bash
cargo bench                          # 全ベンチマーク実行
cargo bench -- parse_host            # 特定ベンチマークのみ
cargo bench -- --save-baseline main  # ベースライン保存
cargo bench -- --baseline main       # ベースラインと比較
```

criterion は統計的に有意な差を検出する。単純な `std::time::Instant` より信頼性が高い。

## Distinguishing Bottlenecks

| Type | Symptom | Fix |
|---|---|---|
| Algorithmic (O(n²)) | 入力サイズで急激に悪化 | アルゴリズム変更 |
| Allocation-heavy | malloc が flamegraph の上位 | `Vec::with_capacity`, arena |
| Memory bandwidth | キャッシュミスが多い | データ構造の局所性改善 |
| Constant factor | 定数倍だが規模は適切 | SIMD, 定数畳み込み |
| I/O bound | CPU 使用率が低い | バッファリング, 並列 I/O |

## Quick Wins for AI-Generated Code

```rust
// Bad: 毎回アロケーション
for item in items {
    let s = format!("{}", item);  // String アロケーション
    process(&s);
}

// Good: 借用で済む
for item in items {
    let s = item.to_string();     // 必要な場合のみ
    process(&item.to_string());   // または直接渡す
}

// Bad: clone() の多用
fn process(data: Vec<u8>) { }    // move が必要か？

// Good: 借用を使う
fn process(data: &[u8]) { }      // スライスで受け取る
```

## When Not to Optimize

- ベンチマークで問題が確認されていない場所
- 1回しか実行されない初期化コード
- テストコード
- エラーパス

「プロファイルが示すまで最適化しない」— Donald Knuth の格言の正しい解釈。
