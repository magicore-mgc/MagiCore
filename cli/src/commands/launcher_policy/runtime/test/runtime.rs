use super::super::super::runtime::{Runtime, dangerous_flags, dangerous_permissions};

#[test]
fn test_runtime_names() {
    assert_eq!(Runtime::Bun.name(), "bun");
    assert_eq!(Runtime::Deno.name(), "deno");
    assert_eq!(Runtime::Node.name(), "node");
}

#[test]
fn test_runtime_flag_profiles_preserve_order() {
    // Bun preloads modules like Node does; the profile must carry it.
    // (Bun preload module như Node.)
    assert_eq!(
        dangerous_flags(Runtime::Bun),
        &[
            "--eval",
            "-e",
            "--print",
            "-p",
            "--preload",
            "-r",
            "--require"
        ]
    );
    // Deno gained module-resolution flags with the same power as `-A`: a remote
    // import map, an arbitrary config, and a module directory all select code to
    // load.
    // (Deno thêm flag resolution ngang quyền `-A`: import map từ xa, config tuỳ ý,
    // thư mục module đều chọn code để nạp.)
    assert_eq!(
        dangerous_flags(Runtime::Deno),
        &[
            "--eval",
            "-e",
            "--allow-all",
            "-A",
            "--import-map",
            "--config",
            "--cert",
            "--node-modules-dir",
            "--seed",
        ]
    );
    // Node and tsx load modules through the same flags, which the profile used to
    // omit — `node --import ./loader.mjs` ran arbitrary code through both
    // `mgc dev` and `mgc run`.
    // (Node/tsx nạp module qua cùng các flag mà profile từng bỏ sót.)
    assert_eq!(
        dangerous_flags(Runtime::Node),
        &[
            "--eval",
            "-e",
            "--print",
            "-p",
            "--import",
            "-r",
            "--require",
            "--loader",
            "--experimental-loader",
            "--experimental-policy",
            "--experimental-permission",
            "--permission",
        ]
    );
    assert_eq!(
        dangerous_flags(Runtime::Tsx),
        dangerous_flags(Runtime::Node),
        "tsx forwards every flag to Node"
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
