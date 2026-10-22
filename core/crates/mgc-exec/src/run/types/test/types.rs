//! Tests for execution-type safety defaults.
//! Test các default an toàn của execution type.

use crate::run::ExecOptions;

#[test]
fn default_options_keep_policy_and_resource_escape_hatches_closed() {
    let options = ExecOptions::default();

    assert!(options.execution_scope.is_none());
    assert!(options.compat_runner.is_none());
    assert!(options.compat_runtime.is_none());
    assert!(options.policy_manifest.is_none());
    assert!(options.policy_context.is_none());
    assert!(options.policy_profile.is_none());
    assert!(options.allowed_exit_codes.is_empty());
    assert!(!options.capture_full_stdout);
    assert!(!options.disable_timeout);
}
