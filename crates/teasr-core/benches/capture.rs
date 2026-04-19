//! Benchmark for terminal capture pipeline.
//!
//! Measures wall time, CPU time, and CPU usage for a representative
//! terminal capture scenario (type a command, wait for output).
//!
//! Run with:
//!   cargo bench -p teasr-core
use std::time::{Duration, Instant};

use teasr_core::backend::CaptureBackend;
use teasr_core::capture::terminal::TerminalBackend;
use teasr_core::types::{CapturedFrame, Interaction};

/// Measure CPU time (user + system) for the current process.
fn cpu_time() -> Duration {
    let mut usage: libc::rusage = unsafe { std::mem::zeroed() };
    unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut usage) };
    let user = Duration::new(
        usage.ru_utime.tv_sec as u64,
        usage.ru_utime.tv_usec as u32 * 1000,
    );
    let sys = Duration::new(
        usage.ru_stime.tv_sec as u64,
        usage.ru_stime.tv_usec as u32 * 1000,
    );
    user + sys
}

struct BenchResult {
    wall: Duration,
    cpu: Duration,
    frames: usize,
    png_bytes: usize,
}

async fn run_terminal_scene(text: &str) -> BenchResult {
    let cpu_start = cpu_time();
    let wall_start = Instant::now();

    let mut backend =
        TerminalBackend::new(80, Some(24), "dracula", None, 42, None, None, None, None);
    backend.setup().await.expect("setup failed");

    let mut frames: Vec<CapturedFrame> = Vec::new();

    // Type the command
    let type_frames = backend
        .execute(&Interaction::Type {
            text: text.to_string(),
            speed: Some(10),
        })
        .await
        .expect("type failed");
    frames.extend(type_frames);

    // Press enter
    let key_frames = backend
        .execute(&Interaction::Key {
            key: "enter".to_string(),
        })
        .await
        .expect("key failed");
    frames.extend(key_frames);

    // Pause to capture command output
    let wait_frames = backend
        .execute(&Interaction::Wait { duration: 2000 })
        .await
        .expect("wait failed");
    frames.extend(wait_frames);

    backend.teardown().await.expect("teardown failed");

    let wall = wall_start.elapsed();
    let cpu = cpu_time() - cpu_start;
    let png_bytes: usize = frames.iter().map(|f| f.png_data.len()).sum();

    BenchResult {
        wall,
        cpu,
        frames: frames.len(),
        png_bytes,
    }
}

fn print_result(name: &str, r: &BenchResult) {
    let cpu_pct = if r.wall.as_secs_f64() > 0.0 {
        r.cpu.as_secs_f64() / r.wall.as_secs_f64() * 100.0
    } else {
        0.0
    };
    println!("  {name}");
    println!("    wall:      {:.2}s", r.wall.as_secs_f64());
    println!("    cpu:       {:.2}s", r.cpu.as_secs_f64());
    println!("    cpu usage: {:.1}%", cpu_pct);
    println!("    frames:    {}", r.frames);
    println!("    png total: {:.1} KB", r.png_bytes as f64 / 1024.0);
    println!();
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    println!();
    println!("teasr capture benchmark");
    println!("{}", "=".repeat(40));
    println!();

    // Benchmark 1: simple echo (minimal output)
    let r1 = rt.block_on(run_terminal_scene("echo 'hello teasr'"));
    print_result("echo (minimal output)", &r1);

    // Benchmark 2: seq 1 50 (moderate output)
    let r2 = rt.block_on(run_terminal_scene("seq 1 50"));
    print_result("seq 1 50 (moderate output)", &r2);

    // Benchmark 3: seq 1 200 (heavy output)
    let r3 = rt.block_on(run_terminal_scene("seq 1 200"));
    print_result("seq 1 200 (heavy output)", &r3);

    println!("{}", "-".repeat(40));
    println!("  all benchmarks passed");
    println!();
}
