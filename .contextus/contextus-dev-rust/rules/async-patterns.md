# Async Patterns (Tokio)

## Task Spawning

```rust
// Independent async task (not CPU-bound)
tokio::spawn(async move {
    process_connection(conn).await;
});

// CPU-bound work: block_in_place or spawn_blocking
let result = tokio::task::spawn_blocking(|| {
    heavy_computation()   // runs on a dedicated thread pool
}).await?;
```

**CPU バウンドな処理を `tokio::spawn` に入れない** — async executor のスレッドをブロックする。

## Channel Patterns

| Pattern | Channel | Use case |
|---|---|---|
| Work queue | `tokio::sync::mpsc` | N producers → 1 worker |
| Fan-out (broadcast) | `tokio::sync::broadcast` | 1 producer → N consumers |
| Single value | `tokio::sync::oneshot` | request/response |
| Semaphore (rate limit) | `tokio::sync::Semaphore` | limit concurrency |

```rust
// mpsc: work queue
let (tx, mut rx) = tokio::sync::mpsc::channel(32);

tokio::spawn(async move {
    while let Some(job) = rx.recv().await {
        process(job).await;
    }
});

tx.send(job).await?;
```

## Graceful Shutdown

```rust
use tokio_util::sync::CancellationToken;

let token = CancellationToken::new();
let child = token.child_token();

tokio::spawn(async move {
    tokio::select! {
        _ = child.cancelled() => { /* cleanup */ }
        _ = do_work() => {}
    }
});

// Signal received
token.cancel();
```

broadcast channel で代替する場合:

```rust
let (shutdown_tx, _) = tokio::sync::broadcast::channel::<()>(1);
let mut rx = shutdown_tx.subscribe();

tokio::select! {
    _ = rx.recv() => { /* shutdown */ }
    _ = serve() => {}
}
```

## Mutex: tokio vs std

```rust
// std::sync::Mutex — .await をまたがない場合はこちら
let shared = Arc::new(std::sync::Mutex::new(state));
{
    let mut guard = shared.lock().unwrap();
    *guard += 1;
    // guard がここでドロップ → .await をまたがない
}
some_async_fn().await;

// tokio::sync::Mutex — .await をまたぐ場合のみ
let shared = Arc::new(tokio::sync::Mutex::new(state));
let mut guard = shared.lock().await;
*guard += 1;
some_async_fn().await;  // guard を保持したまま await できる
```

**使い分け**: `.await` をまたがないなら `std::sync::Mutex`。
`tokio::sync::Mutex` は必要なときだけ使う。

## Common Deadlocks and Pitfalls

| Mistake | Result | Fix |
|---|---|---|
| `std::sync::MutexGuard` を `.await` の前に保持 | デッドロック | スコープで早期ドロップ or `tokio::sync::Mutex` |
| `tokio::sync::MutexGuard` を `Send` 境界で送る | コンパイルエラー | ガードをまたがない設計に変更 |
| blocking I/O を async context で呼ぶ | executor スターベーション | `spawn_blocking` でラップ |
| `async fn` を大量に spawn してチャンネルなし | バックプレッシャーなし | bounded channel + `await` on send |

## Timeout and Race

```rust
use tokio::time::{timeout, Duration};

// タイムアウト
let result = timeout(Duration::from_secs(5), async_op()).await
    .map_err(|_| anyhow::anyhow!("timeout"))?;

// 最初に完了した方を使う
tokio::select! {
    res = primary() => res,
    res = fallback() => res,
}
```
