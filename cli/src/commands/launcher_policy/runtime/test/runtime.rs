use super::super::super::runtime::{Runtime, dangerous_flags, dangerous_permissions};

#[test]
fn test_runtime_names() {
    assert_eq!(Runtime::Bun.name(), "bun");
    assert_eq!(Runtime::Deno.name(), "deno");
    assert_eq!(Runtime::Node.name(), "node");
}

#[test]
fn test_runtime_flag_profiles_preserve_order() {
    assert_eq!(
        dangerous_flags(Runtime::Bun),
        &["--eval", "-e", "--print", "-p"]
    );
    assert_eq!(
        dangerous_flags(Runtime::Deno),
        &["--eval", "-e", "--allow-all", "-A"]
    );
    assert_eq!(
        dangerous_flags(Runtime::Node),
        &["--eval", "-e", "--print", "-p"]
    );
}

#[test]
fn test_deno_permission_profile_preserves_order() {
    assert_eq!(
        dangerous_permissions(Runtime::Deno),
        &[
            "--allow-read",
            "--allow-write",
            "--allow-net",
            "--allow-run",
            "--allow-ffi",
            "--allow-hrtime",
        ]
    );
    assert!(dangerous_permissions(Runtime::Bun).is_empty());
    assert!(dangerous_permissions(Runtime::Node).is_empty());
}
