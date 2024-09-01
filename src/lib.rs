//! Helpers to bridge between sync and async code.

use std::future::Future;
use std::io;

use tokio::{runtime, task};

/// Build same runtime as `#[tokio::main]`.
///
/// # Errors
///
/// Errors if runtime cannot be built.
pub fn make_runtime() -> io::Result<runtime::Runtime> {
    runtime::Builder::new_multi_thread().enable_all().build()
}

/// Run a task inside an async runtime from a blocking context.
///
/// This is when you don't want to be async all the way up, as it can be
/// very invasive. Instead, you can keep the complexity low by having
/// your app be mostly synchronous, while be able to leverage async
/// sparingly where it makes sense.
///
/// <div class="warning">
///
/// This requires an already running async execution environment. The
/// simplest way to achieve this is with the regular `#[tokio::main]`.
/// This `async fn main()` can then call normal synchronous code. Then,
/// to run async code, pass the root task to this function.
///
/// In any case, the runtime _must be multi-threaded_.
///
/// </div>
///
/// # Examples
///
/// ```
/// use tokio_sync2async::sync_await;
///
/// #[tokio::main]
/// async fn main() {
///     sync_fn();
/// }
///
/// fn sync_fn() {
///     let res = sync_await(async_fn());
///     assert_eq!(res, 42);
/// }
///
/// async fn async_fn() -> i32 {
///     42
/// }
/// ```
///
/// # Implementation Details
///
/// - [`task::block_in_place()`] runs the provided blocking closure in a
///   thread that _does not block the async executor_. This ensures that
///   while the blocking operation is executing, the async runtime can
///   continue running other tasks.
/// - [`Handle::current()`](runtime::Handle::current) gives us access to
///   the main runtime.
/// - [`handle.block_on()`](runtime::Handle::block_on) runs the future
///   and blocks until it completes. This is crucial as well. Since the
///   caller is sync, it needs the result to continue execution.
pub fn sync_await<F: Future>(future: F) -> F::Output {
    task::block_in_place(|| {
        let handle = runtime::Handle::current();
        handle.block_on(future)
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unused_async)]

    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_sync_await() {
        fn sync_fn() -> i32 {
            // Equivalent to `async_fn().await` but inside a sync `fn`.
            sync_await(async_fn())
        }

        async fn async_fn() -> i32 {
            42
        }

        assert_eq!(sync_fn(), 42);
    }
}
