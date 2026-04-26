pub(crate) mod chrome;
pub mod screen;
pub mod terminal;
pub mod web;

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime};
use tracing::debug;

/// Default per-character typing delay (milliseconds) when `speed` is omitted.
pub(crate) const DEFAULT_TYPE_SPEED_MS: u64 = 80;

/// Jitter applied to per-keystroke delays as a fraction of the base delay
/// (e.g. 0.2 → ±20%). Breaks the metronome feel of fixed-interval typing.
pub(crate) const TYPE_JITTER_RATIO: f64 = 0.2;

/// Returns a per-keystroke delay around `base_ms` with ±`TYPE_JITTER_RATIO`
/// jitter. Uses a tiny xorshift PRNG seeded from system time on first use,
/// so we don't pull in a `rand` dependency for one helper.
pub(crate) fn humanized_keystroke_delay(base_ms: u64) -> Duration {
    let factor = 1.0 + TYPE_JITTER_RATIO * (next_unit_centered_random());
    let scaled = (base_ms as f64 * factor).max(1.0);
    Duration::from_millis(scaled as u64)
}

/// Returns a value in `[-1.0, 1.0]` from an xorshift64* PRNG.
fn next_unit_centered_random() -> f64 {
    static STATE: AtomicU64 = AtomicU64::new(0);
    let mut s = STATE.load(Ordering::Relaxed);
    if s == 0 {
        s = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x9E3779B97F4A7C15)
            | 1;
    }
    s ^= s << 13;
    s ^= s >> 7;
    s ^= s << 17;
    STATE.store(s, Ordering::Relaxed);
    let u = s.wrapping_mul(0x2545F4914F6CDD1D);
    // Map to [0, 1) then to [-1, 1).
    let unit = (u >> 11) as f64 / (1u64 << 53) as f64;
    unit * 2.0 - 1.0
}

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
