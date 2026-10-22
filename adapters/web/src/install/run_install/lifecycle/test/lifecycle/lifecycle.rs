//! Lifecycle JoinSet drain tests.
//! Test drain JoinSet lifecycle.

#[tokio::test]
async fn lifecycle_failure_does_not_return_before_sibling_tasks_finish() {
    use crate::install::drain_lifecycle_tasks;
    use mgc_types::MgError;
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };
    use tokio::task::JoinSet;

    let sibling_finished = Arc::new(AtomicBool::new(false));
    let mut tasks = JoinSet::new();
    tasks.spawn(async { Err::<(), _>(MgError::Other("injected lifecycle failure".into())) });

    let finished = sibling_finished.clone();
    tasks.spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(25)).await;
        finished.store(true, Ordering::SeqCst);
        Ok::<(), MgError>(())
    });

    let error = drain_lifecycle_tasks(&mut tasks)
        .await
        .expect("the first lifecycle failure must be returned after draining siblings");

    assert!(error.contains("injected lifecycle failure"));
    assert!(
        sibling_finished.load(Ordering::SeqCst),
        "rollback must not start while a sibling lifecycle task can still mutate the tree"
    );
}
