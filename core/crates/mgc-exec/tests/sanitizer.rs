//! Public API route smoke test; detailed cases live beside their source owners.

use mgc_exec::prelude::redact_args;

#[test]
fn sanitizer_is_exported_from_the_public_prelude() {
    assert_eq!(
        redact_args(&["--token=value".to_string()]),
        vec!["--token=[REDACTED]".to_string()]
    );
}
