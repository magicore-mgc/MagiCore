//! Acquire capacity before spawning blocking KDF work, retaining it on cancellation.
//! Giữ suất trước khi spawn KDF blocking và giữ tới khi xong dù request bị hủy.

use std::sync::{Arc, OnceLock};
use tokio::sync::Semaphore;

use super::CredentialError;

const MAX_CONCURRENT_KDFS: usize = 4;
static KDF_CAPACITY: OnceLock<Arc<Semaphore>> = OnceLock::new();

pub(crate) async fn run_kdf<T, F>(work: F) -> Result<T, CredentialError>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, CredentialError> + Send + 'static,
{
    let capacity = KDF_CAPACITY.get_or_init(|| Arc::new(Semaphore::new(MAX_CONCURRENT_KDFS)));
    run_with_capacity(capacity.clone(), work).await
}

pub(super) async fn run_with_capacity<T, F>(
    capacity: Arc<Semaphore>,
    work: F,
) -> Result<T, CredentialError>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, CredentialError> + Send + 'static,
{
    let permit = capacity
        .acquire_owned()
        .await
        .map_err(|_| CredentialError::WorkerUnavailable)?;
    tokio::task::spawn_blocking(move || {
        let _permit = permit;
        work()
    })
    .await
    .map_err(|_| CredentialError::WorkerUnavailable)?
}
