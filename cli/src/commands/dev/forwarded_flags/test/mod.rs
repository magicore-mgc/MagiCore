//! Forwarded-flag validation tests — test kiểm flag chuyển tiếp.
//! A framework CLI forwards argv to Node, so a dangerous Node flag must be refused
//! there too. Before this, `next dev --require evil` reached the binary.
//! (CLI framework chuyển argv cho Node, nên flag Node nguy hiểm phải bị từ chối ở
//! đó. Trước đây `next dev --require evil` tới được binary.)

use super::validate_forwarded_runtime_flags;

/// Every Node-family binary refuses a module-loading flag.
/// (Mọi binary họ Node từ chối flag nạp module.)
#[test]
fn every_node_family_program_refuses_module_loading() {
    for program in ["node", "tsx", "bun"] {
        assert!(
            validate_forwarded_runtime_flags(program, &["--require", "evil"]).is_err(),
            "'{program} --require' must be refused"
        );
    }
}

/// A framework CLI is validated with the Node profile.
/// (CLI framework được kiểm với profile Node.)
#[test]
fn a_node_framework_refuses_the_same_flags() {
    for program in ["next", "vite", "nuxt", "astro", "webpack", "nest"] {
        assert!(
            validate_forwarded_runtime_flags(program, &["--require", "evil"]).is_err(),
            "'{program} --require' must be refused"
        );
        assert!(
            validate_forwarded_runtime_flags(program, &["dev", "--import", "./x.mjs"]).is_err(),
            "'{program} --import' must be refused"
        );
    }
}

/// A subcommand argument is not a flag.
/// (Tham số subcommand không phải flag.)
#[test]
fn ordinary_subcommands_and_flags_pass() {
    for program in ["next", "vite", "node"] {
        validate_forwarded_runtime_flags(program, &["dev", "--port", "4315"])
            .unwrap_or_else(|error| panic!("'{program} dev --port 4315' must pass: {error}"));
    }
}

/// An absolute path still resolves the profile from the file name.
/// (Đường dẫn tuyệt đối vẫn nhận profile từ tên file.)
#[test]
fn an_absolute_path_resolves_to_its_binary_name() {
    assert!(
        validate_forwarded_runtime_flags("/usr/local/bin/node", &["--eval", "1"]).is_err(),
        "the profile comes from the file name, not the whole path"
    );
    #[cfg(windows)]
    assert!(
        validate_forwarded_runtime_flags("C:/bin/node.exe", &["--eval", "1"]).is_err(),
        "a Windows .exe suffix must not hide the profile"
    );
}

/// A non-JS program is left to its own toolchain rules.
/// (Chương trình không phải JS để toolchain của nó xử lý.)
#[test]
fn a_non_js_program_is_not_policed_here() {
    for program in ["cargo", "python3", "go", "dotnet", "php"] {
        validate_forwarded_runtime_flags(program, &["--eval", "x"])
            .unwrap_or_else(|error| panic!("'{program}' must not be policed here: {error}"));
    }
}
