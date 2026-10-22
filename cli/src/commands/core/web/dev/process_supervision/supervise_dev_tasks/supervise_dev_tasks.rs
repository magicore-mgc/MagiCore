//! Cancels sibling dev tasks and reaps every completed task.
//! Hủy task dev cùng nhóm và thu hồi toàn bộ task.
use anyhow::Result;
use std::sync::Arc;

pub(in crate::commands::core::web::implementation) const DEV_CANCEL_POLL_INTERVAL_MS: u64 = 25;
/// Wait for either a process-wide signal or sibling cancellation.
/// Chờ tín hiệu dừng toàn process hoặc yêu cầu hủy từ target cùng nhóm.
pub(in crate::commands::core::web::implementation) async fn wait_for_dev_stop(
    cancel_flag: Arc<std::sync::atomic::AtomicBool>,
) -> Result<()> {
    loop {
        if mgc_exec::prelude::shutdown_requested() {
            return Err(crate::error::web_dev_interrupted());
        }
        if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(crate::error::web_dev_target_cancelled());
        }
        tokio::time::sleep(std::time::Duration::from_millis(
            DEV_CANCEL_POLL_INTERVAL_MS,
        ))
        .await;
    }
}
/// Stop every sibling after the first target finishes, then reap all tasks.
/// Dừng target cùng nhóm khi target đầu tiên kết thúc, sau đó thu hồi hết task.
pub(in crate::commands::core::web::implementation) async fn supervise_dev_tasks(
    tasks: &mut tokio::task::JoinSet<Result<()>>,
    cancel_flag: Arc<std::sync::atomic::AtomicBool>,
) -> Result<()> {
    let Some(first) = tasks.join_next().await else {
        return Ok(());
    };

    cancel_flag.store(true, std::sync::atomic::Ordering::SeqCst);
    let mut first_result = match first {
        Ok(result) => result,
        Err(error) => Err(error.into()),
    };
    let mut reap_error = None;
    while let Some(result) = tasks.join_next().await {
        if let Err(error) = result
            && reap_error.is_none()
        {
            reap_error = Some(anyhow::Error::from(error));
        }
    }

    if first_result.is_ok()
        && let Some(error) = reap_error
    {
        first_result = Err(error);
    }
    first_result
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
