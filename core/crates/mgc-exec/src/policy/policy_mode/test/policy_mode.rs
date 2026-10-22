use crate::policy::PolicyMode;

#[test]
fn native_and_declared_compat_modes_are_distinct() {
    assert_eq!(PolicyMode::Native, PolicyMode::Native);
    let compat = PolicyMode::CompatRunner {
        manager: "npm".to_string(),
    };
    assert_eq!(
        compat,
        PolicyMode::CompatRunner {
            manager: "npm".to_string()
        }
    );
    assert_ne!(compat, PolicyMode::Native);
}
