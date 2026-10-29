//! PHP dev launch port and entrypoint tests — test port và entrypoint dev PHP.
//! Every case asserts the RULE §13 port contract: a default is one of the
//! 4·3·1·5 permutations, and an explicit `--port` always wins.
//! Mỗi case khẳng định contract port RULE §13: giá trị mặc định là một hoán vị
//! 4·3·1·5, và `--port` tường minh luôn thắng.

use std::fs;

use crate::commands::core::dev_port::PORT_WEB_BE;

use super::super::super::super::types::NativeDevContext;
use super::super::infer_php_dev_launch;

fn context<'a>(root: &'a std::path::Path, port: Option<u16>) -> NativeDevContext<'a> {
    NativeDevContext {
        project_root: root,
        host: None,
        port,
    }
}

fn arg_after<'a>(args: &'a [std::ffi::OsString], prefix: &str) -> &'a str {
    args.iter()
        .filter_map(|arg| arg.to_str())
        .find(|arg| arg.starts_with(prefix))
        .unwrap_or_else(|| panic!("missing argument starting with '{prefix}': {args:?}"))
}

/// The Laravel lane must never fall back to a port outside the §13 set.
/// (Lane Laravel không được rơi về port ngoài tập §13.)
#[test]
fn artisan_defaults_to_the_shared_back_end_port() {
    let dir = tempfile::tempdir().expect("temp dir");
    fs::write(dir.path().join("artisan"), "").expect("write artisan marker");

    let launch = infer_php_dev_launch(&context(dir.path(), None))
        .expect("inference succeeds")
        .expect("artisan is a PHP entrypoint");

    assert_eq!(launch.program, std::path::PathBuf::from("php"));
    let port = arg_after(&launch.args, "--port=");
    assert_eq!(port, format!("--port={PORT_WEB_BE}"));
}

/// Artisan receives the port inline; the built-in server receives it in the
/// host binding instead. Both must carry the same resolved number.
/// (Artisan nhận port inline; built-in server nhận port trong host binding.
/// Cả hai phải dùng cùng một số đã resolve.)
#[test]
fn builtin_server_uses_the_same_default_port_in_its_binding() {
    let dir = tempfile::tempdir().expect("temp dir");
    fs::write(dir.path().join("composer.json"), "{}").expect("write composer.json");
    fs::create_dir_all(dir.path().join("public")).expect("create public");
    fs::write(dir.path().join("public/index.php"), "").expect("write front controller");

    let launch = infer_php_dev_launch(&context(dir.path(), None))
        .expect("inference succeeds")
        .expect("composer + public/index.php is a PHP entrypoint");

    let binding = launch
        .args
        .iter()
        .filter_map(|arg| arg.to_str())
        .find(|arg| arg.starts_with("localhost:"))
        .expect("built-in server carries a host:port binding");
    assert_eq!(binding, format!("localhost:{PORT_WEB_BE}"));
}

/// An explicit `--port` from the user always wins over the table default.
/// (`--port` người dùng chọn luôn thắng mặc định trong bảng.)
#[test]
fn explicit_port_overrides_the_default_on_both_entrypoints() {
    let artisan_dir = tempfile::tempdir().expect("temp dir");
    fs::write(artisan_dir.path().join("artisan"), "").expect("write artisan marker");
    let launch = infer_php_dev_launch(&context(artisan_dir.path(), Some(5413)))
        .expect("inference succeeds")
        .expect("artisan entrypoint");
    assert_eq!(arg_after(&launch.args, "--port="), "--port=5413");

    let builtin_dir = tempfile::tempdir().expect("temp dir");
    fs::write(builtin_dir.path().join("composer.json"), "{}").expect("write composer.json");
    fs::create_dir_all(builtin_dir.path().join("public")).expect("create public");
    fs::write(builtin_dir.path().join("public/index.php"), "").expect("write front controller");
    let launch = infer_php_dev_launch(&context(builtin_dir.path(), Some(5413)))
        .expect("inference succeeds")
        .expect("built-in server entrypoint");
    assert!(
        launch
            .args
            .iter()
            .filter_map(|arg| arg.to_str())
            .any(|arg| arg == "localhost:5413"),
        "explicit port must reach the built-in server binding: {:?}",
        launch.args
    );
}

/// Symfony's `bin/console` selects the built-in server even when the project
/// also has a front controller; the fallback path must not need it.
/// (Symfony `bin/console` chọn built-in server kể cả khi có front controller;
/// nhánh fallback không được phụ thuộc file đó.)
#[test]
fn symfony_console_selects_the_builtin_server() {
    let dir = tempfile::tempdir().expect("temp dir");
    fs::write(dir.path().join("composer.json"), "{}").expect("write composer.json");
    fs::create_dir_all(dir.path().join("public")).expect("create public");
    fs::write(dir.path().join("public/index.php"), "").expect("write front controller");
    fs::create_dir_all(dir.path().join("bin")).expect("create bin");
    fs::write(dir.path().join("bin/console"), "").expect("write console");

    let launch = infer_php_dev_launch(&context(dir.path(), None))
        .expect("inference succeeds")
        .expect("symfony entrypoint");
    assert!(
        launch
            .args
            .iter()
            .filter_map(|arg| arg.to_str())
            .any(|arg| arg == "-S"),
        "symfony must run the built-in server: {:?}",
        launch.args
    );
}

/// A directory with neither an Artisan script nor a composer front controller
/// yields no launch so another language launcher can claim the project.
/// (Thư mục không có Artisan lẫn composer front controller trả `None` để launcher
/// ngôn ngữ khác nhận project.)
#[test]
fn a_directory_without_php_entrypoints_yields_none() {
    let dir = tempfile::tempdir().expect("temp dir");
    fs::write(dir.path().join("composer.json"), "{}").expect("write composer.json");

    assert!(
        infer_php_dev_launch(&context(dir.path(), None))
            .expect("inference succeeds")
            .is_none(),
        "composer.json alone is not a PHP dev entrypoint"
    );
}

/// The port contract is a property of the whole table, not of one launcher:
/// assert the resolved default is a permutation of 4·3·1·5.
/// (Contract port là thuộc tính của cả bảng, không của một launcher: kiểm giá
/// trị mặc định là hoán vị 4·3·1·5.)
#[test]
fn default_port_is_a_4315_permutation() {
    let mut digits: Vec<char> = PORT_WEB_BE.to_string().chars().collect();
    digits.sort_unstable();
    assert_eq!(digits, vec!['1', '3', '4', '5']);
}
