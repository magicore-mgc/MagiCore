//! Public policy API smoke test — kiểm tra smoke API policy công khai.

use mgc_exec::policy::{Invocation, PolicyDecision, PolicyMode, resolve};

#[test]
fn public_policy_items_keep_the_existing_module_path() {
    let invocation = Invocation {
        program: "node".into(),
        args: vec!["server.js".into()],
        env: Vec::new(),
        in_compat_subtree: false,
    };
    assert_eq!(
        resolve(&PolicyMode::Native, &invocation, None, None),
        PolicyDecision::Allow,
    );
}
