//! Tests for dependency pipeline error mapping.
//! Test ánh xạ lỗi pipeline dependency.

use std::error::Error as _;
use std::sync::Arc;

use mgc_types::MgError;
use tokio::sync::Semaphore;

use super::super::PipelineError;

#[test]
fn extract_semaphore_failure_preserves_shared_diagnostic() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("test runtime builds");
    let semaphore = Arc::new(Semaphore::new(1));
    semaphore.close();
    let source = runtime
        .block_on(semaphore.acquire_owned())
        .expect_err("closed semaphore rejects acquisition");
    let local = PipelineError::ExtractSemaphoreClosed(source);
    assert!(local.source().is_some());

    let MgError::Other(message) = MgError::from(local) else {
        panic!("extract semaphore failure keeps the shared error category");
    };
    assert!(message.starts_with("extract semaphore closed: "));
}

#[test]
fn extract_join_failure_preserves_shared_diagnostic() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("test runtime builds");
    let source = runtime.block_on(async {
        let task = tokio::spawn(async { std::future::pending::<()>().await });
        task.abort();
        task.await.expect_err("aborted task returns a join error")
    });
    let local = PipelineError::ExtractionTaskPanicked(source);
    assert!(local.source().is_some());

    let MgError::Other(message) = MgError::from(local) else {
        panic!("extract join failure keeps the shared error category");
    };
    assert!(message.starts_with("extract task panicked: "));
}
