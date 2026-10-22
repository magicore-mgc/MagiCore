//! Keep rival-lock guidance stable at the local error boundary.
//! Giữ nguyên hướng dẫn lockfile đối thủ tại ranh giới lỗi cục bộ.

use super::*;
use mgc_types::MgError;

#[test]
fn pin_source_error_preserves_rival_remediation() {
    let error = PinSourceError::RivalLockfiles(vec!["deno.lock".to_string()]);
    let mapped = MgError::from(error);
    assert_eq!(
        mapped.to_string(),
        concat!(
            "rival lockfile(s) detected [deno.lock] but mgc.lock is missing — ",
            "mgc audit consumes mgc.lock ONLY. Run `mgc import deno` to migrate this project first."
        )
    );
}
