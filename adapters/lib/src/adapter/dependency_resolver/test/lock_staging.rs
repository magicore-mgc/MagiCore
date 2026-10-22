//! Verify pending-lock replacement and fail-closed poison handling.
//! Kiểm tra thay pending-lock và xử lý poison theo fail-closed.

use super::super::lock_staging::stage_pending_lock;
use mgc_lockfile::Package;
use std::sync::{Arc, Mutex};

#[test]
fn successful_resolve_replaces_pending_lock_rows() {
    let original = Package {
        name: "old-package".to_owned(),
        ..Package::default()
    };
    let replacement = Package {
        name: "resolved-package".to_owned(),
        ..Package::default()
    };
    let pending = Mutex::new(vec![original]);

    stage_pending_lock(&pending, vec![replacement.clone()]).unwrap();

    assert_eq!(*pending.lock().unwrap(), vec![replacement]);
}

#[test]
fn poisoned_pending_lock_fails_closed() {
    let pending = Arc::new(Mutex::new(Vec::<Package>::new()));
    let poisoned = Arc::clone(&pending);
    let _ = std::thread::spawn(move || {
        let _guard = poisoned.lock().unwrap();
        panic!("poison pending lock for the test");
    })
    .join();

    let error = stage_pending_lock(&pending, Vec::new()).unwrap_err();

    assert_eq!(
        error.to_string(),
        "lib pending lock poisoned — refusing rather than operating on uncertain state"
    );
}
