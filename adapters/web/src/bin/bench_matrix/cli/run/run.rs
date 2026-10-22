use crate::bench_matrix::cli::parse_arguments;
// Benchmark command execution
// Điều phối tham số, matrix, báo cáo và thao tác baseline.
use crate::bench_matrix::*;

pub(crate) fn run() -> anyhow::Result<()> {
    // SAFETY: process init before any thread is spawned (bench harness
    // threads start later) — no concurrent env access can race this write.
    // AN TOÀN: ghi env trước khi spawn thread — không có race đồng thời.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST", "1");
    }
    let options = parse_arguments()?;
    let scenarios = run_matrix(options.runs, options.profile)?;
    print_report(&scenarios);
    if let Some(name) = options.save_baseline {
        save_baseline(&name, &scenarios)?;
    }
    if let Some(name) = options.compare_baseline {
        let baseline = load_baseline(&name)?;
        let has_regression = print_comparison(&baseline.scenarios, &scenarios);
        if has_regression {
            eprintln!("WARN: REGRESSION DETECTED: some scenarios degraded >10% vs baseline");
        }
    }
    Ok(())
}
