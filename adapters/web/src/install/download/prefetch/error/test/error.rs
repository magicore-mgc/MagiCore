//! Tests for prefetch error mapping.
//! Test ánh xạ lỗi prefetch.

use std::error::Error as _;
use std::sync::Arc;

use mgc_types::MgError;
use tokio::sync::Semaphore;

use super::super::PrefetchError;

#[test]
fn semaphore_failure_keeps_other_message_and_source() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("test runtime builds");
    let semaphore = Arc::new(Semaphore::new(1));
    semaphore.close();
    let source = runtime
        .block_on(semaphore.acquire_owned())
        .expect_err("closed semaphore rejects acquisition");
    let local = PrefetchError::SemaphoreClosed(source);
    assert!(local.source().is_some());

    let MgError::Other(message) = MgError::from(local) else {
        panic!("semaphore failure keeps the shared error category");
    };
    assert!(message.starts_with("download semaphore closed: "));
}

#[test]
fn join_failure_keeps_other_message_and_source() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("test runtime builds");
    let source = runtime.block_on(async {
        let task = tokio::spawn(async { std::future::pending::<()>().await });
        task.abort();
        task.await.expect_err("aborted task returns a join error")
    });
    let local = PrefetchError::JoinTask(source);
    assert!(local.source().is_some());

    let MgError::Other(message) = MgError::from(local) else {
        panic!("join failure keeps the shared error category");
    };
    assert!(message.starts_with("download task failed: "));
}
