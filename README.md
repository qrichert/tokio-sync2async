# tokio-sync2async

_Helpers to bridge between sync and async code._

```rust
use tokio_sync2async::sync_await;

#[tokio::main]
async fn main() {
    sync_fn();
}

fn sync_fn() {
    let res = sync_await(async_fn());
    assert_eq!(res, 42);
}

async fn async_fn() -> i32 {
    42
}
```
