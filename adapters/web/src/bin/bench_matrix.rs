// Web benchmark matrix binary entrypoint — Cargo tự phát hiện binary benchmark Web.
// Keep the executable path stable and delegate its command behavior — giữ nguyên đường dẫn binary.
#[path = "bench_matrix/mod.rs"]
mod bench_matrix;

fn main() {
    if let Err(err) = bench_matrix::run() {
        let message = err.to_string();
        if message.contains("Operation not permitted") || message.contains("Permission denied") {
            eprintln!("bench_matrix skipped in restricted sandbox: {message}");
            std::process::exit(0);
        }
        eprintln!("bench_matrix failed: {err}");
        std::process::exit(1);
    }
}
