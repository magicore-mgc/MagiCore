use super::support::user;
use crate::{auth::AuthService, credential_security::run_kdf, storage::RegistryStore};
use std::{sync::Arc, task::Poll};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn revocation_during_a_password_worker_wait_cannot_return_a_stale_identity() {
    let directory = tempfile::tempdir().unwrap();
    let store = Arc::new(RegistryStore::new(directory.path()).await.unwrap());
    let auth = AuthService::new(None, store);
    auth.register_user(
        "revoked-token".into(),
        user("revoked", Some("private-password")),
    )
    .await
    .unwrap();
    let mut releases = Vec::new();
    let mut workers = Vec::new();
    for _ in 0..4 {
        let (entered, started) = tokio::sync::oneshot::channel();
        let (release, wait) = std::sync::mpsc::channel();
        releases.push(release);
        workers.push(tokio::spawn(async move {
            run_kdf(move || {
                let _ = entered.send(());
                let _ = wait.recv();
                Ok(())
            })
            .await
        }));
        started.await.unwrap();
    }
    let mut pending = Box::pin(auth.verify_password_async("revoked", "private-password"));
    let was_waiting = matches!(futures_util::poll!(pending.as_mut()), Poll::Pending);
    let removed = auth.remove_token("revoked-token").await.unwrap();
    for release in releases {
        let _ = release.send(());
    }
    for worker in workers {
        worker.await.unwrap().unwrap();
    }
    assert!(was_waiting);
    assert!(removed);
    assert!(pending.await.is_none());
}
