use crate::credential_security::worker::run_with_capacity;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::sync::Semaphore;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn blocking_work_never_exceeds_its_capacity() {
    let capacity = Arc::new(Semaphore::new(4));
    let active = Arc::new(AtomicUsize::new(0));
    let peak = Arc::new(AtomicUsize::new(0));
    let mut tasks = tokio::task::JoinSet::new();
    for _ in 0..12 {
        let (capacity, active, peak) = (capacity.clone(), active.clone(), peak.clone());
        tasks.spawn(async move {
            run_with_capacity(capacity, move || {
                let count = active.fetch_add(1, Ordering::SeqCst) + 1;
                peak.fetch_max(count, Ordering::SeqCst);
                std::thread::sleep(std::time::Duration::from_millis(10));
                active.fetch_sub(1, Ordering::SeqCst);
                Ok(())
            })
            .await
        });
    }
    while let Some(result) = tasks.join_next().await {
        result.unwrap().unwrap();
    }
    assert!((1..=4).contains(&peak.load(Ordering::SeqCst)));
    assert_eq!(active.load(Ordering::SeqCst), 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn canceled_request_does_not_release_a_running_workers_permit() {
    let capacity = Arc::new(Semaphore::new(1));
    let (entered, started) = tokio::sync::oneshot::channel();
    let (release, wait) = std::sync::mpsc::channel();
    let worker_capacity = capacity.clone();
    let request = tokio::spawn(async move {
        run_with_capacity(worker_capacity, move || {
            let _ = entered.send(());
            let _ = wait.recv();
            Ok(())
        })
        .await
    });
    started.await.unwrap();
    request.abort();
    let _ = request.await;
    let permits_after_cancel = capacity.available_permits();
    let _ = release.send(());
    run_with_capacity(capacity, || Ok(())).await.unwrap();
    assert_eq!(permits_after_cancel, 0);
}
