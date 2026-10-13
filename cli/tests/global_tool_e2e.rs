#![allow(clippy::unwrap_used)]

#[cfg(any(unix, windows))]
mod platform {
    use std::path::Path;
    use std::process::Command;
    use tempfile::TempDir;

    struct RegistryFixture {
        _server: mockito::ServerGuard,
        _mocks: Vec<mockito::Mock>,
        url: String,
    }

    impl RegistryFixture {
        fn new(versions: &[&str]) -> Self {
            Self::new_for_packages(&[("mgc-cli-probe", "mgc-probe", versions)])
        }

        fn new_for_packages(packages: &[(&str, &str, &[&str])]) -> Self {
            let mut server = mockito::Server::new();
            let url = server.url();
            let mut mocks = Vec::new();
            for (package, binary, versions) in packages {
                let versions_json = versions
                    .iter()
                    .map(|version| {
                        (
                            (*version).to_string(),
                            package_metadata(&url, package, binary, version),
                        )
                    })
                    .collect::<serde_json::Map<_, _>>();
                let metadata = serde_json::json!({
                    "name": package,
                    "dist-tags": { "latest": versions.last().unwrap() },
                    "versions": versions_json
                });
                mocks.push(
                    server
                        .mock("GET", format!("/{package}").as_str())
                        .with_status(200)
                        .with_header("content-type", "application/json")
                        .with_body(metadata.to_string())
                        .expect_at_least(1)
                        .create(),
                );
                for version in *versions {
                    mocks.push(
                        server
                            .mock(
                                "GET",
                                format!("/{package}/-/{package}-{version}.tgz").as_str(),
                            )
                            .with_status(200)
                            .with_header("content-type", "application/octet-stream")
                            .with_body(package_tarball(package, binary, version))
                            .expect_at_least(1)
                            .create(),
                    );
                }
            }
            Self {
                _server: server,
                _mocks: mocks,
                url,
            }
        }
    }

    fn package_metadata(
        registry: &str,
        package: &str,
        binary: &str,
        version: &str,
    ) -> serde_json::Value {
        serde_json::json!({
            "name": package,
            "version": version,
            "bin": { binary: "bin/cli" },
            "dependencies": {},
            "dist": {
                "tarball": format!("{registry}/{package}/-/{package}-{version}.tgz")
            }
        })
    }

    fn package_tarball(package: &str, binary: &str, version: &str) -> Vec<u8> {
        let manifest = serde_json::json!({
            "name": package,
            "version": version,
            "bin": { binary: "bin/cli" }
        })
        .to_string();
        let executable = format!(
            "#!/usr/bin/env node\nconsole.log('{}|' + (process.argv[2] || ''));\n",
            version
        );
        let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        let mut archive = tar::Builder::new(encoder);
        append_file(
            &mut archive,
            "package/package.json",
            manifest.as_bytes(),
            0o644,
        );
        append_file(
            &mut archive,
            "package/bin/cli",
            executable.as_bytes(),
            0o755,
        );
        archive.finish().unwrap();
        archive.into_inner().unwrap().finish().unwrap()
    }

    fn append_file(
        archive: &mut tar::Builder<flate2::write::GzEncoder<Vec<u8>>>,
        path: &str,
        content: &[u8],
        mode: u32,
    ) {
        let mut header = tar::Header::new_gnu();
        header.set_mode(mode);
        header.set_size(content.len() as u64);
        header.set_cksum();
        archive.append_data(&mut header, path, content).unwrap();
    }

    fn run_mgc(
        args: &[&str],
        cwd: &Path,
        home: &Path,
        data: &Path,
        registry: Option<&str>,
        success: bool,
    ) -> String {
        let mut command = Command::new(env!("CARGO_BIN_EXE_mgc"));
        command
            .args(args)
            .current_dir(cwd)
            .env("HOME", home)
            .env("USERPROFILE", home)
            .env("SHELL", "/bin/zsh")
            .env("MGC_DATA_DIR", data)
            .env("MAGICORE_WEB_ALLOW_INSECURE_LOCALHOST", "1")
            .env("CARGO_NET_OFFLINE", "true");
        if let Some(registry) = registry {
            command
                .env("MAGICORE_WEB_REGISTRY_URL", registry)
                .env("MAGICORE_WEB_ALLOWED_REGISTRIES", registry);
        }
        let output = command.output().expect("spawn mgc");
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(output.status.success(), success, "mgc {args:?}:\n{text}");
        text
    }

    fn global_bin_path(data: &Path, binary: &str) -> std::path::PathBuf {
        #[cfg(windows)]
        let name = format!("{binary}.cmd");
        #[cfg(not(windows))]
        let name = binary.to_string();
        data.join("bin").join(name)
    }

    fn run_tool(path: &Path, argument: &str) -> std::process::Output {
        #[cfg(windows)]
        {
            Command::new("powershell.exe")
                .args([
                    "-NoLogo",
                    "-NoProfile",
                    "-NonInteractive",
                    "-Command",
                    "& $env:MGC_TEST_TOOL_SHIM $env:MGC_TEST_TOOL_ARGUMENT",
                ])
                .env("MGC_TEST_TOOL_SHIM", path)
                .env("MGC_TEST_TOOL_ARGUMENT", argument)
                .output()
                .expect("run global Windows tool shim")
        }
        #[cfg(not(windows))]
        {
            Command::new(path)
                .arg(argument)
                .output()
                .expect("run global Unix tool shim")
        }
    }

    #[cfg(windows)]
    struct RestoreWindowsUserPath {
        original: Option<String>,
        restored: bool,
    }

    #[cfg(windows)]
    impl RestoreWindowsUserPath {
        fn capture() -> Self {
            let output = Command::new("powershell.exe")
                .args([
                    "-NoLogo",
                    "-NoProfile",
                    "-NonInteractive",
                    "-Command",
                    "$p = [Environment]::GetEnvironmentVariable('Path', 'User'); if ($null -eq $p) { exit 44 }; [Console]::Out.Write($p)",
                ])
                .output()
                .expect("read Windows user PATH");
            let original = match output.status.code() {
                Some(0) => Some(String::from_utf8_lossy(&output.stdout).into_owned()),
                Some(44) => None,
                _ => panic!(
                    "cannot read Windows user PATH: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            };
            Self {
                original,
                restored: false,
            }
        }

        fn restore(&mut self) -> std::io::Result<()> {
            const SCRIPT: &str = r#"if ($env:MGC_TEST_PATH_PRESENT -eq '1') { [Environment]::SetEnvironmentVariable('Path', $env:MGC_TEST_OLD_USER_PATH, 'User') } else { [Environment]::SetEnvironmentVariable('Path', $null, 'User') }"#;
            let mut command = Command::new("powershell.exe");
            command.args([
                "-NoLogo",
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                SCRIPT,
            ]);
            if let Some(original) = &self.original {
                command
                    .env("MGC_TEST_PATH_PRESENT", "1")
                    .env("MGC_TEST_OLD_USER_PATH", original);
            } else {
                command.env("MGC_TEST_PATH_PRESENT", "0");
            }
            let output = command.output()?;
            if !output.status.success() {
                return Err(std::io::Error::other(format!(
                    "restore Windows user PATH: {}",
                    String::from_utf8_lossy(&output.stderr)
                )));
            }
            self.restored = true;
            Ok(())
        }
    }

    #[cfg(windows)]
    impl Drop for RestoreWindowsUserPath {
        fn drop(&mut self) {
            if !self.restored {
                if let Err(error) = self.restore() {
                    eprintln!("failed to restore Windows user PATH after E2E: {error}");
                }
            }
        }
    }

    #[test]
    #[cfg_attr(
        windows,
        ignore = "temporarily changes the Windows user PATH; run explicitly in isolated CI"
    )]
    fn global_cli_install_update_list_and_remove_are_isolated_and_persistent() {
        #[cfg(windows)]
        let mut path_guard = RestoreWindowsUserPath::capture();
        let cwd = TempDir::new().unwrap();
        let home = TempDir::new().unwrap();
        let data = home.path().join("magicore-data");
        let registry = RegistryFixture::new(&["1.0.0", "1.1.0"]);
        let mut caller_config = mgc_config::ProjectConfig::new("caller", "web");
        let mut caller_registry =
            mgc_config::registry::Registry::new("local".to_string(), registry.url.clone());
        caller_registry.token = Some("project-config-secret".to_string());
        caller_registry.username = Some("project-config-user".to_string());
        caller_registry.password = Some("project-config-password".to_string());
        caller_config.registries.push(caller_registry);
        caller_config.save(cwd.path()).unwrap();
        std::fs::write(
            cwd.path().join(".npmrc"),
            "//127.0.0.1/:_authToken=temporary-project-token\n",
        )
        .unwrap();

        run_mgc(
            &[
                "tool",
                "install",
                "npm:mgc-cli-probe@1.0.0",
                "--bin",
                "mgc-probe",
            ],
            cwd.path(),
            home.path(),
            &data,
            Some(&registry.url),
            true,
        );

        let bin = global_bin_path(&data, "mgc-probe");
        let first = run_tool(&bin, "before");
        assert!(first.status.success());
        assert_eq!(
            String::from_utf8_lossy(&first.stdout).trim(),
            "1.0.0|before"
        );
        let listed = run_mgc(
            &["tool", "list"],
            cwd.path(),
            home.path(),
            &data,
            None,
            true,
        );
        assert!(listed.contains("mgc-probe"));
        assert!(listed.contains("mgc-cli-probe@1.0.0"));
        let tool_project = std::fs::read_dir(data.join("tools"))
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        let record_path = tool_project.join(".mgc-tool.json");
        let original_record = std::fs::read_to_string(&record_path).unwrap();
        let mut invalid_record: serde_json::Value = serde_json::from_str(&original_record).unwrap();
        invalid_record["target"] = serde_json::Value::String(String::new());
        std::fs::write(&record_path, invalid_record.to_string()).unwrap();
        let invalid_list = run_mgc(
            &["tool", "list"],
            cwd.path(),
            home.path(),
            &data,
            None,
            false,
        );
        assert!(invalid_list.contains("unsupported schema or identity"));
        std::fs::write(&record_path, &original_record).unwrap();

        let pending_record: serde_json::Value = serde_json::from_str(&original_record).unwrap();
        let project_name = tool_project.file_name().unwrap().to_string_lossy();
        let pending = serde_json::json!({
            "schema": 1,
            "source": "npm",
            "package": "mgc-cli-probe",
            "selector": "1.0.0",
            "project_name": project_name,
            "record": pending_record
        });
        std::fs::remove_file(&record_path).unwrap();
        std::fs::write(
            tool_project.join(".mgc-tool-installing.json"),
            serde_json::to_vec_pretty(&pending).unwrap(),
        )
        .unwrap();
        let incomplete = run_mgc(
            &["tool", "list"],
            cwd.path(),
            home.path(),
            &data,
            None,
            true,
        );
        assert!(incomplete.contains("INCOMPLETE"));
        run_mgc(
            &[
                "tool",
                "install",
                "npm:mgc-cli-probe@1.0.0",
                "--bin",
                "mgc-probe",
            ],
            cwd.path(),
            home.path(),
            &data,
            Some(&registry.url),
            true,
        );
        let recovered = run_tool(&bin, "recovered");
        assert!(recovered.status.success());
        assert_eq!(
            String::from_utf8_lossy(&recovered.stdout).trim(),
            "1.0.0|recovered"
        );
        let persisted_config = std::fs::read_to_string(tool_project.join("mgc.toml")).unwrap();
        assert!(!persisted_config.contains("project-config-secret"));
        assert!(!persisted_config.contains("project-config-user"));
        assert!(!persisted_config.contains("project-config-password"));
        assert!(!tool_project.join(".npmrc").exists());

        let failed_update = run_mgc(
            &["tool", "update", "mgc-probe", "--version", "9.9.9"],
            cwd.path(),
            home.path(),
            &data,
            Some(&registry.url),
            false,
        );
        assert!(failed_update.contains("9.9.9"));
        let after_rollback = run_tool(&bin, "rollback");
        assert!(after_rollback.status.success());
        assert_eq!(
            String::from_utf8_lossy(&after_rollback.stdout).trim(),
            "1.0.0|rollback"
        );

        run_mgc(
            &["tool", "update", "mgc-probe", "--version", "1.1.0"],
            cwd.path(),
            home.path(),
            &data,
            Some(&registry.url),
            true,
        );
        let updated = run_tool(&bin, "after");
        assert!(updated.status.success());
        assert_eq!(
            String::from_utf8_lossy(&updated.stdout).trim(),
            "1.1.0|after"
        );

        #[cfg(unix)]
        {
            let profile = std::fs::read_to_string(home.path().join(".zprofile")).unwrap();
            assert!(
                profile.contains(
                    &data
                        .join("bin")
                        .canonicalize()
                        .unwrap()
                        .display()
                        .to_string()
                )
            );
            assert_eq!(profile.matches("magicore-managed-tool-bin >>>").count(), 1);
        }
        #[cfg(windows)]
        {
            let output = Command::new("powershell.exe")
                .args([
                    "-NoLogo",
                    "-NoProfile",
                    "-NonInteractive",
                    "-Command",
                    "[Console]::Out.Write([Environment]::GetEnvironmentVariable('Path', 'User'))",
                ])
                .output()
                .unwrap();
            assert!(output.status.success());
            let path = String::from_utf8_lossy(&output.stdout).to_ascii_lowercase();
            let managed_bin = data
                .join("bin")
                .canonicalize()
                .unwrap()
                .to_string_lossy()
                .to_ascii_lowercase();
            assert!(
                path.split(';')
                    .any(|part| part.trim_end_matches('\\') == managed_bin.trim_end_matches('\\'))
            );
        }
        assert!(!cwd.path().join("package.json").exists());

        run_mgc(
            &["tool", "remove", "mgc-probe"],
            cwd.path(),
            home.path(),
            &data,
            None,
            true,
        );
        assert!(!bin.exists());
        assert_eq!(std::fs::read_dir(data.join("tools")).unwrap().count(), 0);
        assert!(
            data.join("cache/web").exists(),
            "the shared Web cache remains reusable after removing a tool"
        );
        let unknown = data.join("tools/unrecognized-data");
        std::fs::create_dir(&unknown).unwrap();
        let unknown_list = run_mgc(
            &["tool", "list"],
            cwd.path(),
            home.path(),
            &data,
            None,
            false,
        );
        assert!(unknown_list.contains("unrecognized managed tool data"));
        #[cfg(windows)]
        path_guard.restore().expect("restore Windows user PATH");
    }

    #[test]
    fn global_tool_install_preserves_a_user_owned_command_collision() {
        let cwd = TempDir::new().unwrap();
        let home = TempDir::new().unwrap();
        let data = home.path().join("magicore-data");
        let registry = RegistryFixture::new(&["1.0.0"]);
        let bin = data.join("bin");
        std::fs::create_dir_all(&bin).unwrap();
        let command = global_bin_path(&data, "mgc-probe");
        std::fs::write(&command, "user-owned-command\n").unwrap();

        let failure = run_mgc(
            &[
                "tool",
                "install",
                "npm:mgc-cli-probe@1.0.0",
                "--bin",
                "mgc-probe",
            ],
            cwd.path(),
            home.path(),
            &data,
            Some(&registry.url),
            false,
        );

        assert!(failure.contains("refusing to overwrite"));
        assert_eq!(
            std::fs::read_to_string(command).unwrap(),
            "user-owned-command\n"
        );
        assert_eq!(std::fs::read_dir(data.join("tools")).unwrap().count(), 0);
        assert!(!home.path().join(".zprofile").exists());
    }

    #[test]
    fn different_projects_share_one_user_tool_root_without_removing_each_other() {
        let first_project = TempDir::new().unwrap();
        let second_project = TempDir::new().unwrap();
        let home = TempDir::new().unwrap();
        let data = home.path().join("magicore-data");
        let registry = RegistryFixture::new_for_packages(&[
            ("mgc-cli-probe", "mgc-probe", &["1.0.0"]),
            ("mgc-second-probe", "mgc-second", &["2.0.0"]),
        ]);

        run_mgc(
            &[
                "tool",
                "install",
                "npm:mgc-cli-probe@1.0.0",
                "--bin",
                "mgc-probe",
            ],
            first_project.path(),
            home.path(),
            &data,
            Some(&registry.url),
            true,
        );
        run_mgc(
            &[
                "tool",
                "install",
                "npm:mgc-second-probe@2.0.0",
                "--bin",
                "mgc-second",
            ],
            second_project.path(),
            home.path(),
            &data,
            Some(&registry.url),
            true,
        );

        let first = global_bin_path(&data, "mgc-probe");
        let second = global_bin_path(&data, "mgc-second");
        for (path, expected) in [(&first, "1.0.0|shared"), (&second, "2.0.0|shared")] {
            let output = run_tool(path, "shared");
            assert!(output.status.success());
            assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), expected);
        }
        let listed = run_mgc(
            &["tool", "list"],
            first_project.path(),
            home.path(),
            &data,
            None,
            true,
        );
        assert!(listed.contains("mgc-probe\tnpm:mgc-cli-probe@1.0.0"));
        assert!(listed.contains("mgc-second\tnpm:mgc-second-probe@2.0.0"));
        assert_eq!(std::fs::read_dir(data.join("tools")).unwrap().count(), 2);

        run_mgc(
            &["tool", "remove", "mgc-probe"],
            first_project.path(),
            home.path(),
            &data,
            None,
            true,
        );
        assert!(!first.exists());
        let remaining = run_tool(&second, "still-installed");
        assert!(remaining.status.success());
        assert_eq!(
            String::from_utf8_lossy(&remaining.stdout).trim(),
            "2.0.0|still-installed"
        );
        assert_eq!(std::fs::read_dir(data.join("tools")).unwrap().count(), 1);
    }

    #[test]
    fn global_tool_requires_an_explicit_registry_source() {
        let cwd = TempDir::new().unwrap();
        let home = TempDir::new().unwrap();
        let data = home.path().join("magicore-data");
        let error = run_mgc(
            &["tool", "install", "mgc-cli-probe"],
            cwd.path(),
            home.path(),
            &data,
            None,
            false,
        );
        assert!(error.contains("npm:"));
        assert!(!data.exists());
    }
}
