//! Real Go runtime probe tests.
//! Test real go runtime probe.

use mgc_exec::prelude::*;
use std::fs;

#[cfg(any(unix, windows))]
#[test]
#[ignore = "requires an explicitly configured real Go SDK"]
fn real_go_readonly_run_survives_cold_compiler_processes() {
    let sdk = std::env::var("MGC_E2E_GO_BINARY").expect("MGC_E2E_GO_BINARY is required");
    assert!(
        std::path::Path::new(&sdk).is_file(),
        "Go SDK executable is missing"
    );
    for _ in 0..2 {
        let project = tempfile::tempdir().unwrap();
        fs::write(project.path().join("main.go"),
            "package main\nimport \"fmt\"\nfunc main() { fmt.Println(\"MGC_GO_COLD_RUNTIME_OK\") }\n").unwrap();
        fs::write(
            project.path().join("go.mod"),
            "module example.com/mgc-go-runtime-probe\ngo 1.21\n",
        )
        .unwrap();
        let cache = project.path().join("go-cache");
        fs::create_dir(&cache).unwrap();
        let temporary = project.path().join("go-temp");
        fs::create_dir(&temporary).unwrap();
        let opts = ExecOptions {
            cwd: Some(project.path().to_path_buf()),
            clean_env: true,
            execution_scope: Some(ExecutionScope::DevServer),
            timeout: Some(std::time::Duration::from_secs(120)),
            env: vec![
                ("GOCACHE".into(), cache.to_string_lossy().into_owned()),
                ("TMPDIR".into(), temporary.to_string_lossy().into_owned()),
                ("TMP".into(), temporary.to_string_lossy().into_owned()),
                ("TEMP".into(), temporary.to_string_lossy().into_owned()),
                ("GOPROXY".into(), "off".into()),
                ("GOSUMDB".into(), "off".into()),
                ("GOTOOLCHAIN".into(), "local".into()),
                ("GOENV".into(), "off".into()),
                ("CGO_ENABLED".into(), "0".into()),
            ],
            ..ExecOptions::default()
        };
        let args = vec!["run".into(), "-mod=readonly".into(), "main.go".into()];
        let result = run(&sdk, &args, &opts).expect("cold Go execution must pass native monitor");
        assert!(result.stdout_tail.contains("MGC_GO_COLD_RUNTIME_OK"));
    }
}
