# tokio-sync2async

[![license: MIT](https://img.shields.io/badge/license-MIT-blue)](https://opensource.org/license/mit)
![GitHub Tag](https://img.shields.io/github/v/tag/qrichert/tokio-sync2async?sort=semver&filter=*.*.*&label=release)
[![crates.io](https://img.shields.io/crates/d/tokio-sync2async?logo=rust&logoColor=white&color=orange)](https://crates.io/crates/tokio-sync2async)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/qrichert/tokio-sync2async/run-tests.yml?label=tests)](https://github.com/qrichert/tokio-sync2async/actions)

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
