//! Cargo build argument tests for the dependency-fetch mode.
//! Test tham số `cargo build` theo chế độ lấy dependency.

use super::*;

/// The ONE config that must be absent whenever Cargo may fetch.
/// Cấu hình DUY NHẤT phải vắng mặt mỗi khi Cargo được phép tải.
const REPLACE_WITH: &str = "source.crates-io.replace-with=\"mgc-managed\"";

#[test]
fn default_build_does_not_force_offline() {
    // Regression: the build hardcoded `--offline`, so a crate the managed store
    // did not hold failed with Cargo's `no matching package named 'bevy'` even
    // with network available, and no CLI flag could change it.
    // (Lỗi gốc: build hardcode `--offline` khiến crate chưa có trong store fail
    // dù có mạng.)
    let args = cargo_build_args(DependencyFetch::Online, &[], true);
    assert_eq!(args, ["build", "--locked"]);
    assert!(!args.iter().any(|arg| arg == "--offline"));
}

#[test]
fn offline_flag_keeps_the_fail_closed_offline_build() {
    let args = cargo_build_args(DependencyFetch::Offline, &[], true);
    assert_eq!(args, ["build", "--locked", "--offline"]);
}

#[test]
fn online_build_never_replaces_crates_io_with_the_directory_source() {
    // Regression (P0): an online build still passed `replace-with`, so cargo
    // only ever looked at the managed directory. A directory source cannot
    // fetch, so `mgc build --core lib` could never download a crate even with
    // network available; it failed as if the project were broken.
    // (Lỗi gốc P0: build online vẫn truyền `replace-with` nên cargo chỉ nhìn
    // thư mục managed — nguồn thư mục không tải được, nên build không bao giờ
    // lấy được crate dù có mạng.)
    let args = cargo_build_args(
        DependencyFetch::Online,
        &managed_source_config(DependencyFetch::Online, "/store/cargo/registry/src"),
        true,
    );
    assert!(
        !args.iter().any(|arg| arg == REPLACE_WITH),
        "online cargo must keep the real crates-io registry: {args:?}"
    );
}

#[test]
fn online_build_still_declares_the_managed_directory_as_a_fallback() {
    let config = managed_source_config(DependencyFetch::Online, "/store/cargo/registry/src");
    assert_eq!(
        config,
        [
            "--config",
            "source.mgc-managed.directory=\"/store/cargo/registry/src\""
        ]
    );
}

#[test]
fn offline_build_replaces_crates_io_with_the_managed_directory() {
    let args = cargo_build_args(
        DependencyFetch::Offline,
        &managed_source_config(DependencyFetch::Offline, "/store/cargo/registry/src"),
        true,
    );
    assert_eq!(
        args,
        [
            "--config",
            REPLACE_WITH,
            "--config",
            "source.mgc-managed.directory=\"/store/cargo/registry/src\"",
            "build",
            "--locked",
            "--offline"
        ]
    );
}

#[test]
fn cargo_argv_differs_between_fetch_modes() {
    // One assertion that names the whole difference: offline carries both the
    // replacement and `--offline`; online carries neither, yet both keep
    // `--locked` and the allowlisted `cargo build` verb.
    // (Bám cả khác biệt trong một test: offline có replacement và `--offline`;
    // online không có cả hai; cả hai vẫn giữ `--locked` và verb `cargo build`.)
    let online = cargo_build_args(
        DependencyFetch::Online,
        &managed_source_config(DependencyFetch::Online, "/store/cargo/registry/src"),
        true,
    );
    let offline = cargo_build_args(
        DependencyFetch::Offline,
        &managed_source_config(DependencyFetch::Offline, "/store/cargo/registry/src"),
        true,
    );
    assert_ne!(online, offline);
    for args in [&online, &offline] {
        assert!(args.iter().any(|arg| arg == "--locked"), "{args:?}");
        assert!(args.iter().any(|arg| arg == "build"), "{args:?}");
    }
    assert!(offline.iter().any(|arg| arg == REPLACE_WITH));
    assert!(offline.iter().any(|arg| arg == "--offline"));
    assert!(!online.iter().any(|arg| arg == "--offline"));
}

#[test]
fn managed_source_options_keep_their_order_before_the_verb() {
    // Cargo only accepts `--config` before the subcommand, so the prefix must
    // survive untouched in both modes.
    // (Cargo chỉ nhận `--config` trước subcommand nên prefix phải giữ nguyên.)
    let prefix = vec!["--config".to_string(), REPLACE_WITH.to_string()];
    assert_eq!(
        cargo_build_args(DependencyFetch::Online, &prefix, true),
        ["--config", REPLACE_WITH, "build", "--locked"]
    );
    assert_eq!(
        cargo_build_args(DependencyFetch::Offline, &prefix, true),
        ["--config", REPLACE_WITH, "build", "--locked", "--offline"]
    );
}
