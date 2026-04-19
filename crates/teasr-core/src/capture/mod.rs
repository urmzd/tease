pub(crate) mod chrome;
pub mod screen;
pub mod terminal;
pub mod web;

use std::future::Future;
use std::pin::Pin;
use std::time::{Duration, Instant};
use tracing::debug;

/// Generic idle detection loop. Polls `has_activity` at `poll_interval_ms` and
/// exits early once no activity is detected for `idle_ms`, or after `timeout_ms`.
///
/// Each backend supplies its own detection strategy:
/// - Chrome backends: DOM MutationObserver
/// - Screen backend: raw pixel comparison
/// - Terminal backend: PTY buffer occupancy (uses its own inline loop)
pub(crate) async fn wait_for_idle<'a>(
    timeout_ms: u64,
    idle_ms: u64,
    poll_interval_ms: u64,
    mut has_activity: impl FnMut() -> Pin<Box<dyn Future<Output = bool> + Send + 'a>>,
) {
    let poll_interval = Duration::from_millis(poll_interval_ms);
    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    let idle_limit = if idle_ms == 0 {
        Duration::from_millis(timeout_ms)
    } else {
        Duration::from_millis(idle_ms)
    };
    let mut idle_elapsed = Duration::ZERO;

    while Instant::now() < deadline {
        tokio::time::sleep(poll_interval).await;

        if has_activity().await {
            idle_elapsed = Duration::ZERO;
        } else {
            idle_elapsed += poll_interval;
        }

        if idle_elapsed >= idle_limit {
            debug!("idle after {}ms", idle_elapsed.as_millis());
            break;
        }
    }
}
