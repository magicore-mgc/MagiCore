#[tokio::test]
async fn multi_dev_supervisor_cancels_and_reaps_sibling_targets() {
    let cancel_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let sibling_stopped = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let mut tasks = tokio::task::JoinSet::<anyhow::Result<()>>::new();

    let sibling_cancel = std::sync::Arc::clone(&cancel_flag);
    let sibling_stopped_flag = std::sync::Arc::clone(&sibling_stopped);
    tasks.spawn(async move {
        while !sibling_cancel.load(std::sync::atomic::Ordering::SeqCst) {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        }
        sibling_stopped_flag.store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    });
    tasks.spawn(async { anyhow::bail!("expected dev target failure") });

    let result = super::super::supervise_dev_tasks(&mut tasks, cancel_flag).await;

    assert!(result.is_err_and(|error| error.to_string().contains("expected dev target failure")));
    assert!(sibling_stopped.load(std::sync::atomic::Ordering::SeqCst));
    assert!(tasks.is_empty(), "supervisor must reap every dev task");
}
