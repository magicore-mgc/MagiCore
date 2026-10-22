//! Canary lifecycle E2E cases. / Test E2E lifecycle cho canary.

use crate::native_ownership_matrix_e2e::support::{
    MatrixSandbox, run_product_cli_direct_bounded, write,
};
use std::process::Command;

pub(crate) fn framework_npm_version_probe_uses_mgc_shim_with_canonical_windows_cwd() {
    let root = tempfile::tempdir().expect("create framework probe project");
    std::fs::write(
        root.path().join("mgc.toml"),
        "name = \"framework-probe\"\necosystem = \"web\"\n",
    )
    .expect("write framework probe project config");
    let script = r#"
const assert = require('node:assert/strict');
if (process.platform === 'win32') {
  assert.equal(process.cwd().toLowerCase(), require('node:fs').realpathSync(process.cwd()).toLowerCase());
}
const version = require('node:child_process').execSync('npm --version', { encoding: 'utf8' }).trim();
assert.equal(version, '0.0.0');
console.log('MGC_AUTHENTICATED_NPM_PROBE_OK');
"#;
    // Invoke the product binary directly: wrapping MGC in mgc-exec here creates
    // a second, test-harness process guard which incorrectly authenticates the
    // product's nested npm shim against the test harness image. The MGC process
    // still runs its own production guard around Node and the npm probe.
    // Gọi thẳng binary sản phẩm: bọc MGC bằng mgc-exec tạo guard thứ hai của
    // test harness, khiến guard ngoài đối chiếu nhầm shim npm lồng bên trong
    // với image của test harness. Guard sản phẩm vẫn giám sát Node và npm.
    let (ok, output) =
        run_product_cli_direct_bounded(root.path(), &["exec", "node", "--", "-e", script]);
    assert!(
        ok && output.contains("MGC_AUTHENTICATED_NPM_PROBE_OK"),
        "{output}"
    );
}

pub(crate) fn native_canary_resolves_launchers_without_recording_arguments() {
    let sandbox = MatrixSandbox::new();
    let tools = ["cargo", "cmd", "powershell", "bash"];
    for tool in tools {
        let mut command = Command::new(tool);
        command
            .arg("sensitive-argument-must-not-be-recorded")
            .env("PATH", sandbox.path_env())
            .env("MGC_CANARY_LOG", &sandbox.canary_log);
        #[cfg(windows)]
        command.env("PATHEXT", ".COM;.EXE;.BAT;.CMD");
        let status = command
            .status()
            .unwrap_or_else(|error| panic!("resolve canary for {tool}: {error}"));
        assert!(status.success(), "canary for {tool} must exit successfully");
    }
    let entries = std::fs::read_dir(&sandbox.canary_log)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut recorded = entries
        .iter()
        .map(|entry| std::fs::read_to_string(entry.path()).unwrap())
        .collect::<Vec<_>>();
    recorded.sort();
    assert_eq!(recorded, vec!["bash\n", "cargo\n", "cmd\n", "powershell\n"]);
    assert!(
        recorded
            .iter()
            .all(|record| !record.contains("sensitive-argument"))
    );
}
