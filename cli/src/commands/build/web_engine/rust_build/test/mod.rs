//! Cargo build argument tests for the dependency-fetch mode.
//! Test tham số `cargo build` theo chế độ lấy dependency.

use super::*;

#[test]
fn default_build_does_not_force_offline() {
    // Regression: the build hardcoded `--offline`, so a crate the managed store
    // did not hold failed with Cargo's `no matching package named 'bevy'` even
    // with network available, and no CLI flag could change it.
    // (Lỗi gốc: build hardcode `--offline` khiến crate chưa có trong store fail
    // dù có mạng.)
    let args = cargo_build_args(DependencyFetch::Online, &[]);
    assert_eq!(args, ["build", "--locked"]);
    assert!(!args.iter().any(|arg| arg == "--offline"));
}

#[test]
fn offline_flag_keeps_the_fail_closed_offline_build() {
    let args = cargo_build_args(DependencyFetch::Offline, &[]);
    assert_eq!(args, ["build", "--locked", "--offline"]);
}

#[test]
fn managed_source_options_keep_their_order_before_the_verb() {
    // Cargo only accepts `--config` before the subcommand, so the prefix must
    // survive untouched in both modes.
    // (Cargo chỉ nhận `--config` trước subcommand nên prefix phải giữ nguyên.)
    let prefix = vec![
        "--config".to_string(),
        "source.crates-io.replace-with=\"mgc-managed\"".to_string(),
    ];
    assert_eq!(
        cargo_build_args(DependencyFetch::Online, &prefix),
        [
            "--config",
            "source.crates-io.replace-with=\"mgc-managed\"",
            "build",
            "--locked"
        ]
    );
    assert_eq!(
        cargo_build_args(DependencyFetch::Offline, &prefix),
        [
            "--config",
            "source.crates-io.replace-with=\"mgc-managed\"",
            "build",
            "--locked",
            "--offline"
        ]
    );
}
