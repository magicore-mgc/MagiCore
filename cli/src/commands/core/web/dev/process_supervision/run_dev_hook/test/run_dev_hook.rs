use super::super::run_dev_hook;
use crate::commands::compat::CompatMode;

#[test]
fn missing_hook_is_a_noop() {
    let dir = tempfile::tempdir().expect("temporary project directory");
    run_dev_hook(dir.path(), "predev", &CompatMode::Native, None)
        .expect("missing lifecycle hook is a no-op");
}
