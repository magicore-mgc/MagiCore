use crate::run::ExecReport;

/// Print captured streams only for successful task reports.
pub(crate) fn print_task_outputs(results: &[(usize, Result<ExecReport, String>)]) {
    for (index, result) in results {
        if let Ok(report) = result {
            print_prefixed(
                *index,
                report.stdout_full.as_bytes(),
                report.stderr_tail.as_bytes(),
            );
        }
    }
}

/// Prefix each captured output line while preserving line order and stream routing.
fn print_prefixed(index: usize, out: &[u8], err: &[u8]) {
    for_each_prefixed_line(index, out, |line| println!("{line}"));
    for_each_prefixed_line(index, err, |line| eprintln!("{line}"));
}

fn for_each_prefixed_line(index: usize, bytes: &[u8], mut emit: impl FnMut(String)) {
    let text = String::from_utf8_lossy(bytes);
    for line in text.lines() {
        emit(format!("[{index}] {line}"));
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
