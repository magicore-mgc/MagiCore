//! Bounded product binary invocation helpers. / Helper gọi binary sản phẩm có giới hạn.

use crate::common;

pub(crate) fn run_product_cli_direct_bounded(
    cwd: &std::path::Path,
    args: &[&str],
) -> (bool, String) {
    use std::time::Duration;

    let mut options = mgc_exec::run::ExecOptions {
        cwd: Some(cwd.to_path_buf()),
        timeout: Some(Duration::from_secs(60)),
        ..Default::default()
    };
    common::configure_test_env(&mut options);
    common::run_product_binary_direct_bounded(
        &std::path::PathBuf::from(mgc_binary()),
        args,
        &options,
    )
}

pub(crate) fn mgc_binary() -> String {
    if let Some(value) = std::env::var_os("MGC_TEST_CLI_BINARY") {
        let binary = std::path::PathBuf::from(value);
        assert!(
            binary.is_file() || cfg!(windows) && binary.with_extension("exe").is_file(),
            "MGC_TEST_CLI_BINARY does not identify an executable: {}",
            binary.display()
        );
        #[cfg(windows)]
        let binary = if binary.is_file() {
            binary
        } else {
            binary.with_extension("exe")
        };
        eprintln!(
            "MagiCore native-ownership product binary: {}",
            binary.display()
        );
        return binary.to_string_lossy().into_owned();
    }
    std::env::var_os("CARGO_BIN_EXE_mgc")
        .map(std::path::PathBuf::from)
        .map(|p| p.to_string_lossy().to_string())
        .expect("CARGO_BIN_EXE_mgc unavailable — run via cargo test")
}
