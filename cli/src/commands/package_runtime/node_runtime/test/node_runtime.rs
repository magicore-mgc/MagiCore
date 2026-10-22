//! Focused node runtime regression tests.
//! Test hồi quy tập trung cho node runtime.

use super::*;

#[test]
#[ignore = "requires host Node.js; run explicitly in the Node runtime verification lane"]
fn node_launcher_preserves_commonjs_and_esm_entrypoint_semantics() {
    let node = resolve_host_node_binary(&std::env::var_os("PATH").unwrap_or_default())
        .expect("host Node.js is required for this runtime test");
    let execution = tempfile::tempdir().expect("execution workspace");
    for (extension, package_type, commonjs) in [
        ("cjs", "module", true),
        ("mjs", "commonjs", false),
        ("js", "commonjs", true),
        ("js", "module", false),
    ] {
        let package = tempfile::tempdir().expect("installed package fixture");
        std::fs::write(
            package.path().join("package.json"),
            serde_json::json!({"type": package_type}).to_string(),
        )
        .expect("write package type");
        let entrypoint = package.path().join(format!("entry ' quoted.{extension}"));
        let source = if commonjs {
            "const assert = require('node:assert/strict');\nassert.equal(require.main, module);\nassert.equal(require.main.filename, __filename);\nconsole.log(JSON.stringify({entry: __filename, args: process.argv.slice(2), cwd: process.cwd()}));\n"
        } else {
            "import { fileURLToPath } from 'node:url';\nawait Promise.resolve();\nconsole.log(JSON.stringify({entry: fileURLToPath(import.meta.url), args: process.argv.slice(2), cwd: process.cwd()}));\n"
        };
        std::fs::write(&entrypoint, source).expect("write entrypoint");
        let entrypoint = entrypoint
            .canonicalize()
            .expect("verified canonical entrypoint");
        let launcher =
            create_node_launcher(execution.path(), &entrypoint).expect("create Node launcher");
        let output = std::process::Command::new(&node)
            .arg(launcher.path())
            .args(["argument with spaces", "--fixture-option"])
            .current_dir(execution.path())
            .output()
            .expect("run host Node.js");
        assert!(
            output.status.success(),
            "{extension}/{package_type} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let result: serde_json::Value =
            serde_json::from_slice(&output.stdout).expect("entrypoint JSON result");
        assert_eq!(result["entry"], entrypoint.to_string_lossy().as_ref());
        assert_eq!(
            result["args"],
            serde_json::json!(["argument with spaces", "--fixture-option"])
        );
        assert_eq!(
            result["cwd"],
            execution
                .path()
                .canonicalize()
                .expect("canonical execution path")
                .to_string_lossy()
                .as_ref()
        );
    }
}

#[test]
fn javascript_bin_targets_are_detected_for_node_runtime_invocation() {
    for file in ["nest.js", "create-nuxt.mjs", "legacy.cjs"] {
        assert!(
            is_node_script(Path::new(file)),
            "{file} is a Node entrypoint"
        );
    }
    for file in ["native-tool", "tool.exe", "tool.sh", "tool.ts"] {
        assert!(
            !is_node_script(Path::new(file)),
            "{file} is not a JS entrypoint"
        );
    }
}

#[test]
fn node_runtime_is_resolved_from_absolute_host_path_entries() {
    let host_bin = tempfile::tempdir().expect("host runtime fixture");
    #[cfg(windows)]
    let executable = host_bin.path().join("node.exe");
    #[cfg(not(windows))]
    let executable = host_bin.path().join("node");
    std::fs::write(&executable, b"node runtime fixture").expect("write runtime fixture");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(&executable)
            .expect("runtime metadata")
            .permissions();
        permissions.set_mode(0o700);
        std::fs::set_permissions(&executable, permissions).expect("mark runtime executable");
    }

    let host_path = std::env::join_paths([host_bin.path()]).expect("host PATH");
    assert_eq!(
        resolve_host_node_binary(&host_path).expect("resolve host Node"),
        executable.canonicalize().expect("canonical runtime")
    );

    assert!(resolve_host_node_binary(std::ffi::OsStr::new("relative-runtime-dir")).is_err());
}

#[test]
fn node_launcher_keeps_the_installed_initializer_path_inside_quoted_source() {
    let execution = tempfile::tempdir().expect("initializer execution workspace");
    let entrypoint = execution
        .path()
        .join("node_modules/@scope/create-kit/bin/entry'; process.exit(9);.mjs");
    let launcher = create_node_launcher(execution.path(), &entrypoint)
        .expect("create contained Node launcher");
    let source = std::fs::read_to_string(launcher.path()).expect("read Node launcher");
    let serialized = serde_json::to_string(entrypoint.to_string_lossy().as_ref())
        .expect("serialize initializer path");

    assert!(launcher.path().starts_with(execution.path()));
    assert!(source.contains(&format!("const entry = {serialized};")));
    assert!(source.contains("process.argv[1] = entry;"));
    // The hostile segment is part of the fixture path, so it legitimately occurs
    // inside the JSON-encoded literal. What must NOT happen is a second,
    // unquoted copy that the Node parser would execute as code: count every
    // occurrence in the emitted source and require it to equal exactly the
    // occurrences inside the JSON literal. A naive single-quote interpolation
    // would emit an extra copy and fail this count.
    // Đoạn hostile nằm trong fixture path nên xuất hiện hợp lệ bên trong literal
    // JSON. Điều cấm là bản copy thứ hai không quoting để Node thực thi như code:
    // đếm mọi occurrence trong source sinh ra và bắt bằng đúng số occurrence
    // trong literal JSON. Nội suy single-quote ngây thơ sẽ sinh thêm bản copy
    // và rớt assertion này.
    let hostile = "process.exit(9);";
    let in_json = serialized.matches(hostile).count();
    assert!(
        in_json > 0,
        "fixture must really contain the hostile segment"
    );
    assert_eq!(
        source.matches(hostile).count(),
        in_json,
        "hostile segment must occur only inside the JSON-encoded entry literal"
    );
}
