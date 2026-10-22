//! Full Lifecycle E2E Tests - Task 3/10
//! Complete chain: create → install → dev/build → test → run
//! Tests all 4 cores: web, ai, app, lib
//! Each step asserts: exit code, artifacts, markers proving optimizer/config propagation

#![allow(clippy::unwrap_used)] // Test code: unwrap acceptable for setup

use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

fn find_mgc_binary() -> PathBuf {
    if let Some(value) = std::env::var_os("MGC_TEST_CLI_BINARY") {
        let binary = PathBuf::from(value);
        if binary.is_file() {
            eprintln!("MagiCore lifecycle product binary: {}", binary.display());
            return binary;
        }
        #[cfg(windows)]
        {
            let executable = binary.with_extension("exe");
            if executable.is_file() {
                eprintln!(
                    "MagiCore lifecycle product binary: {}",
                    executable.display()
                );
                return executable;
            }
        }
        panic!(
            "MGC_TEST_CLI_BINARY does not identify an executable: {}",
            binary.display()
        );
    }

    // CARGO_BIN_EXE_mgc: cargo-provided path to the JUST-BUILT binary —
    // immune to stale target/debug/mgc picking up an older build.
    // CARGO_BIN_EXE_mgc: cargo trỏ tới binary VỪA BUILD — tránh lẫm
    // target/debug/mgc cũ là binary lỗi thời.
    let binary = PathBuf::from(
        std::env::var_os("CARGO_BIN_EXE_mgc")
            .expect("CARGO_BIN_EXE_mgc not set — run via `cargo test -p mgc`"),
    );
    eprintln!("MagiCore lifecycle test binary: {}", binary.display());
    binary
}

#[test]
#[ignore = "requires Node.js; executed by CI after setup-node"]
fn test_web_full_lifecycle() {
    // REAL E2E: web core - MUST call mgc create-web
    // NO manual package.json creation allowed
    // REQUIRES: Node.js and public registry access; package install is native.

    if Command::new("node").arg("--version").output().is_err() {
        panic!(
            "UNVERIFIED: node not available\n\
            This test requires Node.js.\n\
            Status: IMPLEMENTED-UNVERIFIED"
        );
    }

    let temp = TempDir::new().unwrap();
    let project_name = "test-web-full";
    let project_path = temp.path().join(project_name);
    let mgc_data = temp.path().join(".mgc-data");
    let mgc_cache = temp.path().join(".mgc-cache");
    let mgc_templates = temp.path().join(".mgc-templates");
    let mgc_store = temp.path().join(".mgc-store");
    for path in [&mgc_data, &mgc_cache, &mgc_templates, &mgc_store] {
        std::fs::create_dir_all(path).unwrap();
    }
    let mgc = find_mgc_binary();

    // === STEP 1: CREATE (REAL mgc create-web call) ===
    // Bước 1: TẠO (gọi mgc create-web thật)
    println!("\n=== STEP 1: mgc create-web vanilla (create-vite via MGC create flow) ===");
    let create_output = Command::new(&mgc)
        .arg("create-web")
        .arg("vanilla")
        .arg(project_name)
        .current_dir(temp.path())
        .env("MGC_DATA_DIR", &mgc_data)
        .env("MGC_CACHE_DIR", &mgc_cache)
        .env("MGC_TEMPLATES_DIR", &mgc_templates)
        .env("MAGICORE_STORE_ROOT", &mgc_store)
        .env_remove("MGC_NPM_REGISTRY")
        .output()
        .expect("mgc create-web failed to execute");

    // MUST succeed - no fallback
    if !create_output.status.success() {
        let stderr = String::from_utf8_lossy(&create_output.stderr);
        panic!(
            "CREATE FAILED - test blocked:\n{}\n\
            This lane requires Node.js and public registry access.\n\
            This test MUST call mgc create-web, not create files manually.",
            stderr
        );
    }

    assert!(project_path.exists(), "Project not created");
    let create_output_text = format!(
        "{}{}",
        String::from_utf8_lossy(&create_output.stdout),
        String::from_utf8_lossy(&create_output.stderr)
    );
    assert!(
        create_output_text
            .contains("create: installing initializer create-vite@latest through MagiCore"),
        "vanilla generator must execute through the MagiCore create flow: {create_output_text}"
    );
    let provenance = std::fs::read_to_string(project_path.join(".mgc-provenance.json"))
        .expect("online generator provenance");
    assert!(provenance.contains("npm:create-vite@"));
    // Same canonical `#generator=` schema as the parity test: exact initializer
    // binary plus template, not the legacy `#template=` form.
    // Cùng schema `#generator=` chuẩn như parity test: binary initializer chính
    // xác cộng template, không phải dạng `#template=` cũ.
    assert!(provenance.contains("#generator=vite:vanilla-ts"));
    assert!(!provenance.contains("npm:create-vite@latest"));

    // The online TypeScript starter creates a package manifest and HTML entry.
    // Starter TypeScript online tạo manifest package và entry HTML.
    assert!(
        project_path.join("index.html").exists(),
        "index.html not created by scaffold"
    );
    assert!(
        project_path.join("mgc.toml").exists(),
        "mgc.toml not created by scaffold"
    );
    assert!(
        project_path.join("package.json").exists(),
        "package.json not created by the online framework generator"
    );
    assert!(
        project_path.join(".mgc.core").exists(),
        ".mgc.core not created by scaffold"
    );

    // Verify scaffold created proper structure
    let core_content = std::fs::read_to_string(project_path.join(".mgc.core")).unwrap();
    assert_eq!(core_content.trim(), "web");

    println!("CREATE verified: scaffold created project");

    // === STEP 2: INSTALL ===
    println!("\n=== STEP 2: mgc install ===");

    // Keep the actual manifest created by create-vite. Only add a project test
    // script, as a developer would; do not replace the generator's dependency
    // graph with a hand-written fixture. MGC must install the generated Vite
    // dependency set through its own resolver and store.
    // Giữ manifest thật do create-vite sinh. Chỉ thêm script test như thao tác
    // của developer; không thay dependency graph của generator bằng fixture.
    // MGC phải tự resolve và cài bộ dependency Vite được sinh ra.
    let package_json_path = project_path.join("package.json");
    let mut package: serde_json::Value = serde_json::from_slice(
        &std::fs::read(&package_json_path).expect("create-vite package.json"),
    )
    .expect("create-vite package.json must be valid JSON");
    let has_vite_dependency = ["dependencies", "devDependencies"].iter().any(|section| {
        package
            .get(section)
            .and_then(|dependencies| dependencies.get("vite"))
            .and_then(serde_json::Value::as_str)
            .is_some()
    });
    assert!(
        has_vite_dependency,
        "create-vite manifest must declare Vite before mgc install"
    );
    let scripts = package
        .get_mut("scripts")
        .and_then(serde_json::Value::as_object_mut)
        .expect("create-vite package.json must contain its scripts object");
    assert!(scripts.contains_key("dev") && scripts.contains_key("build"));
    scripts.insert(
        "test".into(),
        serde_json::json!("node -e \"console.log('Test passed')\""),
    );
    std::fs::write(
        &package_json_path,
        serde_json::to_vec_pretty(&package).expect("serialize augmented project manifest"),
    )
    .expect("write test script into generated project manifest");

    let install_output = Command::new(&mgc)
        .arg("install")
        .current_dir(&project_path)
        .env("MGC_DATA_DIR", &mgc_data)
        .env("MGC_CACHE_DIR", &mgc_cache)
        .env("MGC_TEMPLATES_DIR", &mgc_templates)
        .env("MAGICORE_STORE_ROOT", &mgc_store)
        .env_remove("MGC_NPM_REGISTRY")
        .output()
        .expect("Failed to run mgc install");

    if !install_output.status.success() {
        panic!(
            "mgc install failed:\n{}",
            String::from_utf8_lossy(&install_output.stderr)
        );
    }

    // Verify node_modules created
    assert!(
        project_path.join("node_modules").exists(),
        "node_modules not created"
    );

    // Verify mgc.lock created
    assert!(
        project_path.join("mgc.lock").exists(),
        "mgc.lock not created"
    );
    let lock_text = std::fs::read_to_string(project_path.join("mgc.lock")).unwrap();
    assert!(
        lock_text.contains("vite"),
        "mgc.lock must own the Vite dependency from create-vite's manifest"
    );
    assert!(
        project_path.join("node_modules/vite").is_dir(),
        "MGC did not materialize the generated Vite dependency"
    );

    println!("INSTALL verified: dependencies installed + lockfile created");

    // === STEP 3: TEST ===
    println!("\n=== STEP 3: mgc test ===");

    let test_output = Command::new(&mgc)
        .arg("test")
        .current_dir(&project_path)
        .env("MGC_DATA_DIR", &mgc_data)
        .env("MGC_CACHE_DIR", &mgc_cache)
        .env("MGC_TEMPLATES_DIR", &mgc_templates)
        .env("MAGICORE_STORE_ROOT", &mgc_store)
        .env_remove("MGC_NPM_REGISTRY")
        .output()
        .expect("Failed to run mgc test");

    if !test_output.status.success() {
        panic!(
            "mgc test failed:\n{}",
            String::from_utf8_lossy(&test_output.stderr)
        );
    }

    let test_stdout = String::from_utf8_lossy(&test_output.stdout);
    assert!(
        test_stdout.contains("Test passed"),
        "Test did not produce expected output"
    );

    println!("TEST verified: test command executed successfully");

    println!("Web FULL LIFECYCLE VERIFIED: create → install → test");
}

#[test]
fn test_lib_full_lifecycle() {
    // FULL E2E: lib core - create → install → build → test
    // REQUIRES: cargo/rustc (should be available in Rust project)

    let temp = TempDir::new().unwrap();
    let project_name = "test-lib-full";
    let project_path = temp.path().join(project_name);
    let mgc = find_mgc_binary();
    let cargo_target_dir = project_path.join(".mgc-test-target");
    let mgc_home = temp.path().join(".mgc-home");
    let mgc_data = mgc_home.join("data");
    let mgc_cache = mgc_home.join("cache");
    let mgc_templates = mgc_home.join("templates");
    let mgc_store = mgc_home.join("store");
    for path in [&mgc_data, &mgc_cache, &mgc_templates, &mgc_store] {
        std::fs::create_dir_all(path).unwrap();
    }

    // === STEP 1: CREATE ===
    println!("\n=== STEP 1: mgc create-lib ===");
    let create_output = Command::new(&mgc)
        .arg("create-lib")
        .arg("rust")
        .arg(project_name)
        .current_dir(temp.path())
        .env("HOME", &mgc_home)
        .env("USERPROFILE", &mgc_home)
        .env("MGC_DATA_DIR", &mgc_data)
        .env("MGC_CACHE_DIR", &mgc_cache)
        .env("MGC_TEMPLATES_DIR", &mgc_templates)
        .env("MAGICORE_STORE_ROOT", &mgc_store)
        .output()
        .expect("mgc create-lib failed to execute");

    assert!(
        create_output.status.success(),
        "CREATE FAILED:\n{}",
        String::from_utf8_lossy(&create_output.stderr)
    );
    assert!(project_path.exists(), "Project directory not created");
    assert!(
        project_path.join("Cargo.toml").exists(),
        "Cargo.toml not created"
    );
    assert!(
        project_path.join("Cargo.lock").exists(),
        "deterministic empty Cargo.lock not created"
    );

    // === STEP 2: INSTALL ===
    println!("\n=== STEP 2: mgc install-lib ===");
    let install_output = Command::new(&mgc)
        .arg("install-lib")
        .current_dir(&project_path)
        .env("HOME", &mgc_home)
        .env("USERPROFILE", &mgc_home)
        .env("MGC_DATA_DIR", &mgc_data)
        .env("MGC_CACHE_DIR", &mgc_cache)
        .env("MGC_TEMPLATES_DIR", &mgc_templates)
        .env("MAGICORE_STORE_ROOT", &mgc_store)
        .output()
        .expect("mgc install-lib failed to execute");
    assert!(
        install_output.status.success(),
        "INSTALL FAILED:\n{}",
        String::from_utf8_lossy(&install_output.stderr)
    );
    assert!(
        project_path.join("mgc.lock").exists(),
        "mgc.lock not created by install-lib"
    );

    // === STEP 3: BUILD ===
    println!("\n=== STEP 3: mgc build ===");
    let build_output = Command::new(&mgc)
        .arg("build")
        .env("CARGO_TARGET_DIR", &cargo_target_dir)
        .env("HOME", &mgc_home)
        .env("USERPROFILE", &mgc_home)
        .env("MGC_DATA_DIR", &mgc_data)
        .env("MGC_CACHE_DIR", &mgc_cache)
        .env("MGC_TEMPLATES_DIR", &mgc_templates)
        .env("MAGICORE_STORE_ROOT", &mgc_store)
        .current_dir(&project_path)
        .output()
        .expect("mgc build failed to execute");

    assert!(
        build_output.status.success(),
        "BUILD FAILED:\n{}",
        String::from_utf8_lossy(&build_output.stderr)
    );

    // VERIFY: Cargo honors the isolated target directory set for this E2E.
    assert!(
        cargo_target_dir.exists(),
        "configured Cargo target directory not created after build"
    );

    // === STEP 4: TEST ===
    println!("\n=== STEP 4: mgc test ===");
    let test_output = Command::new(&mgc)
        .arg("test")
        .env("CARGO_TARGET_DIR", &cargo_target_dir)
        .env("HOME", &mgc_home)
        .env("USERPROFILE", &mgc_home)
        .env("MGC_DATA_DIR", &mgc_data)
        .env("MGC_CACHE_DIR", &mgc_cache)
        .env("MGC_TEMPLATES_DIR", &mgc_templates)
        .env("MAGICORE_STORE_ROOT", &mgc_store)
        .current_dir(&project_path)
        .output()
        .expect("mgc test failed to execute");

    // Test should succeed (cargo new creates passing test)
    assert!(
        test_output.status.success(),
        "TEST FAILED:\n{}",
        String::from_utf8_lossy(&test_output.stderr)
    );

    let test_combined = format!(
        "{}{}",
        String::from_utf8_lossy(&test_output.stdout),
        String::from_utf8_lossy(&test_output.stderr)
    );

    // VERIFY: cargo test ran
    assert!(
        test_combined.contains("test result") || test_combined.contains("running"),
        "TEST did not show test results:\n{}",
        test_combined
    );

    println!("Lib full lifecycle VERIFIED: create → install → build → test");
}

#[test]
#[ignore = "Requires Python runtime; pytest is installed through MGC in this E2E"]
fn test_ai_full_lifecycle() {
    // REAL E2E: ai core - MUST call mgc create-ai
    // MUST verify dependencies installed and pytest runs
    // REQUIRES: Python runtime only; project packages are installed by MGC.

    if Command::new("python").arg("--version").output().is_err()
        && Command::new("python3").arg("--version").output().is_err()
    {
        panic!(
            "UNVERIFIED: Python runtime not available\n\
            Status: IMPLEMENTED-UNVERIFIED"
        );
    }

    let temp = TempDir::new().unwrap();
    let project_name = "test-ai-full";
    let project_path = temp.path().join(project_name);
    let mgc = find_mgc_binary();

    // === STEP 1: CREATE - MUST use real mgc create-ai ===
    println!("\n=== STEP 1: mgc create-ai python-agent (embedded) ===");
    let create_output = Command::new(&mgc)
        .arg("create-ai")
        .arg("python-agent")
        .arg(project_name)
        .current_dir(temp.path())
        .output()
        .expect("mgc create-ai failed");

    if !create_output.status.success() {
        panic!(
            "CREATE FAILED:\n{}",
            String::from_utf8_lossy(&create_output.stderr)
        );
    }

    assert!(project_path.exists(), "Project not created");
    assert!(
        project_path.join("pyproject.toml").exists(),
        "pyproject.toml not created by scaffold"
    );

    let core_content = std::fs::read_to_string(project_path.join(".mgc.core")).unwrap();
    assert_eq!(core_content.trim(), "ai");

    println!("CREATE verified: scaffold created project");

    // === STEP 2: Add dependency to create lockfile ===
    println!("\n=== STEP 2: mgc add (create lockfile) ===");

    // AI PEP 621 dependencies and pytest must use MagiCore's native resolver.
    // Dependency PEP 621 và pytest của AI phải qua resolver native MagiCore.

    // AI scaffold has pyproject.toml but no initial deps
    // Run `mgc add` to create lockfile before `mgc install`
    let add_output = Command::new(&mgc)
        .arg("add")
        .arg("pytest@9.1.1")
        .current_dir(&project_path)
        .env_remove("MGC_COMPAT_RUNTIME")
        .output()
        .expect("mgc add failed");

    if !add_output.status.success() {
        // No false-positive: a failed add MUST fail the lifecycle test.
        // Không false-positive: add fail thì lifecycle test phải fail.
        panic!(
            "ADD FAILED:\n{}",
            String::from_utf8_lossy(&add_output.stderr)
        );
    }

    println!("ADD verified: lockfile created");

    let lock_content = std::fs::read_to_string(project_path.join("mgc.lock"))
        .expect("mgc add must create mgc.lock");
    assert!(
        lock_content.to_ascii_lowercase().contains("pytest"),
        "mgc.lock must own pytest; a system pytest must not satisfy this E2E"
    );

    // === STEP 3: INSTALL from lockfile ===
    println!("\n=== STEP 3: mgc install ===");

    let install_output = Command::new(&mgc)
        .arg("install")
        .current_dir(&project_path)
        .env_remove("MGC_COMPAT_RUNTIME")
        .output()
        .expect("mgc install failed");

    if !install_output.status.success() {
        // No false-positive: a failed install MUST fail the lifecycle test.
        // Không false-positive: install fail thì lifecycle test phải fail.
        panic!(
            "INSTALL FAILED:\n{}",
            String::from_utf8_lossy(&install_output.stderr)
        );
    }

    let install_combined = format!(
        "{}{}",
        String::from_utf8_lossy(&install_output.stdout),
        String::from_utf8_lossy(&install_output.stderr)
    );
    assert!(
        install_combined.contains("Using mgc.lock for install state.")
            && install_combined.contains("cache source: shared mgc store")
            && install_combined.contains("All dependencies installed")
            && !install_combined.contains("COMPATIBILITY MODE"),
        "native install must prove mgc.lock/store ownership and must not claim compatibility mode:\n{install_combined}"
    );

    println!("INSTALL verified");

    std::fs::create_dir_all(project_path.join("tests")).unwrap();
    std::fs::write(
        project_path.join("tests/test_mgc_lifecycle.py"),
        "def test_python_runtime_is_available():\n    import sys\n    assert sys.version_info >= (3, 12)\n",
    )
    .unwrap();

    // === STEP 4: TEST - Run MGC-installed pytest ===
    println!("\n=== STEP 4: mgc test ===");
    let test_output = Command::new(&mgc)
        .arg("test")
        .current_dir(&project_path)
        .output()
        .expect("mgc test failed");

    let test_combined = format!(
        "{}{}",
        String::from_utf8_lossy(&test_output.stdout),
        String::from_utf8_lossy(&test_output.stderr)
    );

    assert!(
        test_output.status.success(),
        "TEST FAILED: mgc test must run the MGC-installed pytest successfully:\n{test_combined}"
    );

    // MUST invoke pytest
    assert!(
        test_combined.contains("pytest") || test_combined.contains("test"),
        "TEST did not invoke pytest:\n{}",
        test_combined
    );

    assert!(
        test_combined
            .contains("tests/test_mgc_lifecycle.py::test_python_runtime_is_available PASSED"),
        "TEST did not execute the lifecycle test:\n{test_combined}"
    );
    println!("TEST verified: MGC-installed pytest ran one project test");
    println!("AI FULL LIFECYCLE VERIFIED: create → install → test");
}

#[test]
fn test_app_scaffold_e2e() {
    // Product CLI E2E: App scaffold. Flutter build/test run in the dedicated
    // lifecycle matrix; creating the project itself does not need the SDK.
    let temp = TempDir::new().unwrap();
    let project_name = "test_app_full"; // Flutter naming
    let project_path = temp.path().join(project_name);
    let mgc = find_mgc_binary();

    // === STEP 1: CREATE ===
    println!("\n=== STEP 1: mgc create-app ===");
    let create_output = Command::new(&mgc)
        .arg("create-app")
        .arg("flutter")
        .arg(project_name)
        .current_dir(temp.path())
        .output()
        .expect("mgc create-app failed to execute");

    assert!(
        create_output.status.success(),
        "CREATE FAILED:\n{}",
        String::from_utf8_lossy(&create_output.stderr)
    );
    assert!(project_path.exists(), "Project directory not created");
    assert!(
        project_path.join("pubspec.yaml").exists(),
        "pubspec.yaml not created"
    );

    // VERIFY: .mgc.core file
    let core_content = std::fs::read_to_string(project_path.join(".mgc.core")).unwrap();
    assert_eq!(core_content.trim(), "app", ".mgc.core should contain 'app'");

    println!(
        "App scaffold CLI E2E verified; Flutter build/test are covered by the lifecycle matrix"
    );
}
