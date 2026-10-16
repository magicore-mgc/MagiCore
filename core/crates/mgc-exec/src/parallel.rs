//! Parallel script orchestration — điều phối script song song.
//!
//! Minimal `concurrently` semantics for native mode (Spec rev4 §2/§4):
//! launch every command through the shared executor, prefix captured output,
//! and — with `kill_others` — cancel sibling process trees after a failure.
//! (Semantics `concurrently` tối thiểu cho native mode: chạy từng lệnh qua
//! executor chung, thêm prefix output và hủy cây tiến trình sibling khi fail.)
//!
//! Scope limits of this slice (giới hạn của slice này):
//! - Finite tasks only (build/test-style); not a dev-server supervisor.
//!   (Chỉ task hữu hạn; chưa phải supervisor cho dev server.)
//! - Output is captured with the executor's bounded buffers. Stderr contains
//!   the standard tail; stdout uses the opt-in bounded full capture.
//!   (Output dùng buffer có giới hạn của executor; stderr là tail, stdout là
//!   full capture có giới hạn.)
//! - The caller still supplies the already-guarded environment.
//!   (Caller vẫn truyền environment đã được guard.)

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use anyhow::{Result, bail};

use super::run::ExecOptions;

/// One parallel command — một lệnh song song.
#[derive(Debug, Clone)]
pub struct ParallelTask {
    /// Program as invoked — chương trình được gọi.
    pub program: String,
    /// Arguments — tham số.
    pub args: Vec<String>,
}

/// Parallel execution options — tùy chọn chạy song song.
#[derive(Debug, Clone)]
pub struct ParallelOptions {
    /// Stop remaining tasks when one fails (concurrently semantics).
    /// Dừng task còn lại khi một task fail.
    pub kill_others: bool,
    /// Prefix captured lines with `[index]` — thêm prefix `[index]` cho output.
    pub prefix_output: bool,
}

impl Default for ParallelOptions {
    fn default() -> Self {
        Self {
            kill_others: true,
            prefix_output: true,
        }
    }
}

/// Per-task outcome in task order — kết quả từng task theo thứ tự task.
#[derive(Debug, Clone)]
pub struct ParallelResult {
    /// Task index — chỉ số task.
    pub index: usize,
    /// Exit code — mã thoát.
    pub exit_code: i32,
    /// Captured stdout — stdout đã thu.
    pub stdout: String,
    /// Captured stderr — stderr đã thu.
    pub stderr: String,
}

/// Launch every task through the guarded executor and fail on the first failure.
/// Chạy mọi task qua executor có guard và fail khi task đầu tiên lỗi.
///
/// Every invocation is preflighted before any task starts, then checked again
/// immediately before its own spawn to close the scheduling window.
/// (Mọi invocation được preflight trước khi chạy task nào, rồi kiểm tra lại
/// ngay trước spawn để đóng khoảng trễ do scheduler.)
pub fn run_parallel(
    tasks: &[ParallelTask],
    opts: &ExecOptions,
    popts: &ParallelOptions,
) -> Result<Vec<ParallelResult>> {
    if tasks.is_empty() {
        return Ok(Vec::new());
    }

    let mut parallel_opts = (*opts).clone();
    parallel_opts
        .execution_scope
        .get_or_insert(crate::allowlist::ExecutionScope::BuildRunner);
    // Preflight every task first — no task starts unless all direct invocations pass.
    // Preflight toàn bộ task — không chạy task nào nếu invocation trực tiếp chưa pass.
    for task in tasks {
        if !super::run::compat_runner_declared(&task.program, opts) {
            let _ = super::run::check_invocation_policy(&task.program, &task.args, &parallel_opts)?;
        }
    }

    let cancellation = AtomicBool::new(false);
    let first_failure = AtomicUsize::new(usize::MAX);
    let task_results = std::thread::scope(|threads| {
        let mut handles = Vec::with_capacity(tasks.len());
        for (index, task) in tasks.iter().enumerate() {
            let cancellation = &cancellation;
            let first_failure = &first_failure;
            let parallel_opts = &parallel_opts;
            handles.push(threads.spawn(move || {
                let mut task_opts = parallel_opts.clone();
                task_opts.capture_full_stdout = true;
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    super::run::run_cancellable_capture(
                        &task.program,
                        &task.args,
                        &task_opts,
                        cancellation,
                    )
                }))
                .map_err(|_| "parallel task executor panicked".to_string())
                .and_then(|result| result.map_err(|error| format!("{error:#}")));

                let failed = match &result {
                    Ok(report) => {
                        report.exit_code != 0
                            && !parallel_opts.allowed_exit_codes.contains(&report.exit_code)
                    }
                    Err(_) => true,
                };
                if failed {
                    let _ = first_failure.compare_exchange(
                        usize::MAX,
                        index,
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                    );
                    if popts.kill_others {
                        cancellation.store(true, Ordering::SeqCst);
                    }
                }
                (index, result)
            }));
        }

        handles
            .into_iter()
            .enumerate()
            .map(|(index, handle)| match handle.join() {
                Ok(result) => result,
                Err(_) => {
                    let _ = first_failure.compare_exchange(
                        usize::MAX,
                        index,
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                    );
                    if popts.kill_others {
                        cancellation.store(true, Ordering::SeqCst);
                    }
                    (index, Err("parallel task worker panicked".to_string()))
                }
            })
            .collect::<Vec<_>>()
    });

    if popts.prefix_output {
        for (index, result) in &task_results {
            if let Ok(report) = result {
                print_prefixed(
                    *index,
                    report.stdout_full.as_bytes(),
                    report.stderr_tail.as_bytes(),
                );
            }
        }
    }

    let failed_index = first_failure.load(Ordering::SeqCst);
    if failed_index != usize::MAX {
        let Some((index, result)) = task_results
            .iter()
            .find(|(index, _)| *index == failed_index)
        else {
            bail!("parallel task supervisor lost the first failing task");
        };
        match result {
            Ok(report) => bail!(
                "parallel task {index} failed with exit code {}",
                report.exit_code
            ),
            Err(error) => bail!("parallel task {index} failed: {error}"),
        }
    }

    task_results
        .into_iter()
        .map(|(index, result)| {
            let report = result.map_err(|error| {
                anyhow::anyhow!("parallel task {index} failed unexpectedly: {error}")
            })?;
            Ok(ParallelResult {
                index,
                exit_code: report.exit_code,
                stdout: report.stdout_full,
                stderr: report.stderr_tail,
            })
        })
        .collect()
}

/// Stream one task's captured bytes with an index prefix.
/// In output đã thu của một task kèm prefix index.
fn print_prefixed(index: usize, out: &[u8], err: &[u8]) {
    for line in String::from_utf8_lossy(out).lines() {
        println!("[{index}] {line}");
    }
    for line in String::from_utf8_lossy(err).lines() {
        eprintln!("[{index}] {line}");
    }
}
