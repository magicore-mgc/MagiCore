//! Tests cho lockfile import (npm/pnpm/yarn/bun → mgc.lock v2).
//! Toàn bộ hermetic: fixture inline, không mạng, không gọi PM nào.

use mgc_lockfile::{import_file, import_into_lockfile};
use tempfile::TempDir;

fn write_lock(dir: &std::path::Path, name: &str, content: &str) -> std::path::PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, content).expect("test operation must succeed");
    path
}

// ---------------------------------------------------------------- npm

const NPM_V3: &str = r#"{
  "name": "app", "lockfileVersion": 3,
  "packages": {
    "": { "name": "app", "dependencies": { "foo": "^1.0.0" } },
    "node_modules/foo": {
      "version": "1.2.3",
      "resolved": "https://registry.example/foo/-/foo-1.2.3.tgz",
      "integrity": "sha512-AAAAfoo",
      "dependencies": { "bar": "^2.0.0" }
    },
    "node_modules/bar": {
      "version": "2.0.1",
      "resolved": "https://registry.example/bar/-/bar-2.0.1.tgz",
      "integrity": "sha512-BAAAbar"
    },
    "node_modules/foo/node_modules/bar": {
      "version": "2.9.9",
      "resolved": "https://registry.example/bar/-/bar-2.9.9.tgz",
      "integrity": "sha512-nested"
    },
    "node_modules/@scope/pkg": {
      "version": "0.1.0",
      "resolved": "https://registry.example/@scope/pkg.tgz",
      "integrity": "sha512-scoped"
    },
    "node_modules/link-dep": { "resolved": "packages/local", "link": true }
  }
}"#;

#[test]
fn npm_v3_imports_packages_and_root_level_edges() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(tmp.path(), "package-lock.json", NPM_V3);

    let (lock, report) = import_file(&path).expect("test operation must succeed");
    assert_eq!(report.source_file, "package-lock.json");

    // link entry không có version bị bỏ; root "" bị bỏ
    assert_eq!(
        report.packages, 4,
        "foo, bar(root), bar(nested), @scope/pkg"
    );
    assert_eq!(lock.packages.len(), 4);

    // sorted theo name
    let names: Vec<_> = lock.packages.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, vec!["@scope/pkg", "bar", "bar", "foo"]);

    let foo = lock
        .packages
        .iter()
        .find(|p| p.name == "foo")
        .expect("test operation must succeed");
    assert_eq!(foo.version, "1.2.3");
    assert_eq!(foo.integrity, "sha512-AAAAfoo");
    // edge chỉ khi lookup thấy đúng node_modules/<dep> ở cấp gốc
    assert_eq!(foo.dependencies, vec!["bar@2.0.1".to_string()]);
}

#[test]
fn npm_unknown_newer_version_imports_by_shape_with_warning() {
    // Chính sách: KHÔNG pin version PM (họ bump liên tục) — shape đúng là import được,
    // version chưa test chỉ sinh cảnh báo.
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(
        tmp.path(),
        "package-lock.json",
        r#"{ "lockfileVersion": 99, "packages": {
          "node_modules/x": { "version": "1.0.0", "resolved": "https://r.example/x.tgz", "integrity": "sha512-x" } } }"#,
    );
    let (_, report) = import_file(&path).expect("test operation must succeed");
    assert_eq!(report.packages, 1);
    assert!(
        report.warnings.iter().any(|w| w.contains("99")),
        "phải cảnh báo version chưa test: {:?}",
        report.warnings
    );
}

#[test]
fn npm_known_version_has_no_warnings() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(tmp.path(), "package-lock.json", NPM_V3);
    let (_, report) = import_file(&path).expect("test operation must succeed");
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
}

#[test]
fn npm_missing_packages_map_rejected_by_structure() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(
        tmp.path(),
        "package-lock.json",
        r#"{ "lockfileVersion": 1, "dependencies": { "foo": { "version": "1.0.0" } } }"#,
    );
    let err = import_file(&path).expect_err("test operation must fail");
    assert!(err.to_string().contains("'packages' map"), "{err}");
}

// ---------------------------------------------------------------- pnpm

const PNPM_V9: &str = r#"lockfileVersion: '9.0'

settings:
  autoInstallPeers: true

packages:
  /ansi-styles@4.3.0(such@range):
    resolution: {integrity: sha512-ZB123pnpm}
  /chalk@5.3.0:
    resolution: {integrity: sha512-dITSchalk}
    dependencies:
      ansi-styles: 4.3.0
      missing-pkg: 9.9.9
"#;

#[test]
fn pnpm_v9_imports_with_integrity_and_exact_edges() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(tmp.path(), "pnpm-lock.yaml", PNPM_V9);

    let (lock, report) = import_file(&path).expect("test operation must succeed");
    assert_eq!(report.packages, 2);
    assert!(
        report.warnings.is_empty(),
        "'9.0' đã tested → không cảnh báo"
    );
    let chalk = lock
        .packages
        .iter()
        .find(|p| p.name == "chalk")
        .expect("test operation must succeed");
    assert_eq!(chalk.version, "5.3.0");
    assert_eq!(chalk.integrity, "sha512-dITSchalk");
    // edge exact "4.3.0" giữ lại; "9.9.9" không có trong tập → bỏ
    assert_eq!(chalk.dependencies, vec!["ansi-styles@4.3.0".to_string()]);

    let styles = lock
        .packages
        .iter()
        .find(|p| p.name == "ansi-styles")
        .expect("test operation must succeed");
    assert_eq!(styles.version, "4.3.0", "hậu tố (such@range) phải được cắt");
    assert_eq!(styles.integrity, "sha512-ZB123pnpm");
}

#[test]
fn pnpm_unknown_version_imports_by_shape_with_warning() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(
        tmp.path(),
        "pnpm-lock.yaml",
        r#"lockfileVersion: '10.0'
packages:
  /x@1.0.0:
    resolution: {integrity: sha512-x}
"#,
    );
    let (_, report) = import_file(&path).expect("test operation must succeed");
    assert_eq!(report.packages, 1);
    assert!(
        report.warnings.iter().any(|w| w.contains("10.0")),
        "phải cảnh báo: {:?}",
        report.warnings
    );
}

#[test]
fn pnpm_two_documents_uses_project_graph_not_env() {
    // Rival-grounded (pnpm ≥10 two-doc files; Deno documents the trap):
    // a single-doc parse silently returns the FIRST (env) document. mgc
    // parses every document and imports the LAST (project graph); the env
    // document is reported, never silently merged.
    // (File hai document: parse đơn-doc âm thầm trả document ĐẦU (env).
    // mgc parse mọi document và nhập CUỐI (graph project).)
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(
        tmp.path(),
        "pnpm-lock.yaml",
        r#"---
lockfileVersion: '9.0'
importers:
  .:
    configDependencies: {}
    packageManagerDependencies:
      pnpm:
        specifier: 11.26.0
        version: 11.26.0
packages:
  /pnpm@11.26.0:
    resolution: {integrity: sha512-envpnpm}
    name: pnpm
    version: 11.26.0
snapshots: {}
---
lockfileVersion: '9.0'
settings:
  autoInstallPeers: true
importers:
  .:
    dependencies:
      react:
        specifier: ^19.0.0
        version: 19.0.0
packages:
  /react@19.0.0:
    resolution: {integrity: sha512-react190}
snapshots: {}
"#,
    );
    let (lock, report) = import_file(&path).expect("test operation must succeed");
    assert!(
        lock.packages.iter().any(|p| p.name == "react"),
        "project graph must be imported: {:?}",
        lock.packages.iter().map(|p| &p.name).collect::<Vec<_>>()
    );
    assert!(
        !lock.packages.iter().any(|p| p.name == "pnpm"),
        "env document must NOT leak into the project graph"
    );
    assert!(
        report
            .warnings
            .iter()
            .any(|w| w.contains("LAST (project graph)")),
        "env document must be reported: {:?}",
        report.warnings
    );
}

#[test]
fn pnpm_missing_packages_map_rejected() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(
        tmp.path(),
        "pnpm-lock.yaml",
        "lockfileVersion: '9.0'\nimporters:\n  .:\n    dependencies: {}\n",
    );
    let err = import_file(&path).expect_err("test operation must fail");
    assert!(err.to_string().contains("'packages' map"), "{err}");
}

// ---------------------------------------------------------------- yarn

const YARN_CLASSIC: &str = r##"# THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY.
# yarn lockfile v1

"@scope/lib@^1.0.0":
  version "1.2.3"
  resolved "https://registry.example/@scope/lib/-/lib-1.2.3.tgz"
  integrity sha512-Yscoped

left-pad@1.3.0:
  version "1.3.0"
  resolved "https://registry.example/left-pad/-/left-pad-1.3.0.tgz"
  integrity sha1-yarnleftpad
"##;

#[test]
fn yarn_classic_imports_scoped_and_plain() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(tmp.path(), "yarn.lock", YARN_CLASSIC);

    let (lock, report) = import_file(&path).expect("test operation must succeed");
    assert_eq!(report.packages, 2);

    let scoped = lock
        .packages
        .iter()
        .find(|p| p.name == "@scope/lib")
        .expect("test operation must succeed");
    assert_eq!(scoped.version, "1.2.3");
    assert_eq!(scoped.integrity, "sha512-Yscoped");

    let plain = lock
        .packages
        .iter()
        .find(|p| p.name == "left-pad")
        .expect("test operation must succeed");
    assert_eq!(plain.version, "1.3.0");
    assert_eq!(
        plain.resolved,
        "https://registry.example/left-pad/-/left-pad-1.3.0.tgz"
    );
}

#[test]
fn yarn_berry_metadata_rejected() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(
        tmp.path(),
        "yarn.lock",
        "__metadata:\n  version: 10\n\"a@npm:1\":\n  version: 1.0.0\n",
    );
    let err = import_file(&path).expect_err("test operation must fail");
    assert!(err.to_string().contains("berry"), "{err}");
}

// ---------------------------------------------------------------- bun

const BUN_JSON: &str = r#"{
  "workspaces": {},
  "packages": {
    "zod": ["zod@3.22.4", "https://registry.example/zod-3.22.4.tgz", "sha512-bunZod"],
    "@x/kit": ["@x/kit@0.2.0"],
    "no-integrity": ["no-integrity@1.0.0", "https://registry.example/ni.tgz"]
  }
}"#;

#[test]
fn bun_json_imports_array_entries() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(tmp.path(), "bun.lock", BUN_JSON);

    let (lock, report) = import_file(&path).expect("test operation must succeed");
    assert_eq!(report.packages, 3);

    let zod = lock
        .packages
        .iter()
        .find(|p| p.name == "zod")
        .expect("test operation must succeed");
    assert_eq!(zod.version, "3.22.4");
    assert_eq!(zod.resolved, "https://registry.example/zod-3.22.4.tgz");
    assert_eq!(zod.integrity, "sha512-bunZod");

    // entry thiếu tarball/integrity vẫn nhập với chuỗi rỗng
    let kit = lock
        .packages
        .iter()
        .find(|p| p.name == "@x/kit")
        .expect("test operation must succeed");
    assert_eq!(kit.version, "0.2.0");
    assert_eq!(kit.resolved, "");
}

#[test]
fn bun_comments_rejected_with_clear_error() {
    // Comments are stripped before parsing; an incomplete document must
    // still fail with the importer’s JSONC diagnostic.
    // Comment được cắt trước khi parse; tài liệu thiếu vẫn phải báo lỗi JSONC.
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(
        tmp.path(),
        "bun.lock",
        "{\n  // comment\n  \"packages\": {\n",
    );

    let err = import_file(&path).expect_err("test operation must fail");
    assert!(err.to_string().contains("not valid JSONC"), "{err}");
}

#[test]
fn bun_jsonc_trailing_commas_and_comments_import() {
    // Real bun writer shape: JSONC trailing comma + line comment —
    // import phải chấp nhận (P0-3: fixture migration bun thật).
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(
        tmp.path(),
        "bun.lock",
        "{\n  \"lockfileVersion\": 1,\n  // writer comment\n  \"packages\": {\n    \"lodash\": [\"lodash@4.17.20\", \"\", {}, \"sha512-x\"],\n  }\n}",
    );

    let (lock, report) = import_file(&path).expect("test operation must succeed");
    assert_eq!(report.packages, 1);
    assert_eq!(lock.packages[0].name, "lodash");
    assert_eq!(lock.packages[0].version, "4.17.20");
}

#[test]
fn bun_rejects_unknown_lockfile_version() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(
        tmp.path(),
        "bun.lock",
        r#"{"lockfileVersion":3,"packages":{"x":["x@1.0.0"]}}"#,
    );

    let err = import_file(&path).expect_err("test operation must fail");
    assert!(err.to_string().contains("lockfileVersion 3"), "{err}");
}

// ---------------------------------------------------------------- detect + priority + e2e

#[test]
fn detect_lists_existing_files_in_priority_order() {
    let tmp = TempDir::new().expect("test operation must succeed");
    write_lock(tmp.path(), "yarn.lock", YARN_CLASSIC);
    write_lock(tmp.path(), "package-lock.json", NPM_V3);

    let detected = mgc_lockfile::detect_legacy_lockfiles(tmp.path());
    let names: Vec<_> = detected.iter().map(|l| l.file_name).collect();
    assert_eq!(names.first().copied(), Some("package-lock.json"));
}

#[test]
fn import_into_lockfile_prefers_npm_over_yarn() {
    let tmp = TempDir::new().expect("test operation must succeed");
    write_lock(tmp.path(), "yarn.lock", YARN_CLASSIC);
    write_lock(tmp.path(), "package-lock.json", NPM_V3);

    let (_, report) = import_into_lockfile(tmp.path()).expect("test operation must succeed");
    assert_eq!(report.source_file, "package-lock.json");
}

#[test]
fn imported_output_is_deterministic_sorted() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(tmp.path(), "package-lock.json", NPM_V3);

    let (a, _) = import_file(&path).expect("test operation must succeed");
    let (b, _) = import_file(&path).expect("test operation must succeed");
    // generated_at luôn khác nhau nên so phần nội dung quyết định
    assert_eq!(
        a.packages, b.packages,
        "import 2 lần phải ra cùng danh sách package"
    );
    // importer ghi lock v3 (schema mới nhất) — export of record
    assert_eq!(a.version, "3");
}

// --------------------------------------------- audit round: adversarial cases

#[test]
fn pnpm_registry_path_dep_spec_resolves_edge() {
    // deps value dạng registry path "/name@ver" cũng phải dựng được edge
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(
        tmp.path(),
        "pnpm-lock.yaml",
        r#"lockfileVersion: '9.0'
packages:
  /a@1.0.0:
    resolution: {integrity: sha512-a}
  /b@2.0.0:
    resolution: {integrity: sha512-b}
    dependencies:
      a: /a@1.0.0
"#,
    );
    let (lock, _) = import_file(&path).expect("test operation must succeed");
    let b = lock
        .packages
        .iter()
        .find(|p| p.name == "b")
        .expect("test operation must succeed");
    assert_eq!(b.dependencies, vec!["a@1.0.0".to_string()]);
}

#[test]
fn npm_same_name_version_dedupes() {
    // root-level và nested trùng (name,version) → chỉ giữ 1
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(
        tmp.path(),
        "package-lock.json",
        r#"{
  "lockfileVersion": 3,
  "packages": {
    "node_modules/dup": { "version": "1.0.0", "resolved": "https://r.example/dup.tgz", "integrity": "sha512-x" },
    "node_modules/other/node_modules/dup": { "version": "1.0.0", "resolved": "https://r.example/dup.tgz", "integrity": "sha512-x" }
  }
}"#,
    );
    let (_, report) = import_file(&path).expect("test operation must succeed");
    assert_eq!(report.packages, 1, "trùng (name,version) phải dedupe");
}

#[test]
fn yarn_multi_spec_header_uses_first() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(
        tmp.path(),
        "yarn.lock",
        "\"multi@^1, multi@^2\":\n  version \"1.5.0\"\n  resolved \"https://r.example/multi.tgz\"\n",
    );
    let (lock, report) = import_file(&path).expect("test operation must succeed");
    assert_eq!(report.packages, 1);
    assert_eq!(lock.packages[0].name, "multi");
    assert_eq!(lock.packages[0].version, "1.5.0");
}

// ---------------------------------------------------------------- deno.lock (P0-3 2026-09-10)

const DENO_V5: &str = r#"{
  "version": "5",
  "specifiers": {},
  "npm": {
    "lodash@4.17.20": { "integrity": "sha512-denoldash" },
    "@scope/kit@0.2.0": { "integrity": "sha512-denokit" }
  },
  "jsr": {
    "@std/bytes@0.224.0": { "integrity": "sha512-jstd" }
  }
}"#;

#[test]
fn deno_lock_imports_npm_pins_with_integrity() {
    // P0-3: `mgc import deno` — npm map vào Package chuẩn (name, version,
    // integrity), JSR giữ tiền tố "jsr:" để audit báo skipped trung thực.
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(tmp.path(), "deno.lock", DENO_V5);

    let (lock, report) = import_file(&path).expect("test operation must succeed");
    assert_eq!(report.packages, 3);

    let lodash = lock
        .packages
        .iter()
        .find(|p| p.name == "lodash")
        .expect("test operation must succeed");
    assert_eq!(lodash.version, "4.17.20");
    assert_eq!(lodash.integrity, "sha512-denoldash");

    let kit = lock
        .packages
        .iter()
        .find(|p| p.name == "@scope/kit")
        .expect("test operation must succeed");
    assert_eq!(kit.version, "0.2.0");

    let jsr = lock
        .packages
        .iter()
        .find(|p| p.name == "jsr:@std/bytes")
        .expect("JSR pin phải được import với tiền tố jsr: (báo skipped trung thực)");
    assert_eq!(jsr.version, "0.224.0");
}

#[test]
fn deno_lock_missing_npm_map_rejected_by_structure() {
    // Fail-closed: deno.lock thiếu map "npm" (schema lạ) → từ chối đoán.
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(tmp.path(), "deno.lock", r#"{"version": "5", "jsr": {}}"#);

    let err = import_file(&path).expect_err("test operation must fail");
    assert!(err.to_string().contains("no 'npm' map"), "{err}");
}

#[test]
fn deno_lock_invalid_json_rejected() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(tmp.path(), "deno.lock", "{ not json");

    let err = import_file(&path).expect_err("test operation must fail");
    assert!(err.to_string().contains("not valid JSON"), "{err}");
}

// ---------------------------------------------------------------- P0-5 lossless (bun/deno)

/// Bun INFO production dependency names are imported as graph edges;
/// peer/platform metadata must never be misread as package dependencies.
#[test]
fn bun_import_preserves_dependency_edges_and_records_skips() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(
        tmp.path(),
        "bun.lock",
        r#"{
          "lockfileVersion": 2,
          "packages": {
            "lodash": ["lodash@4.17.20", "https://registry.npmjs.org/lodash/-/lodash-4.17.20.tgz", {
              "dependencies": {"dep-a": "^1.0.0", "dep-b": "^2.0.0"},
              "peerDependencies": {"peer-a": "*"},
              "optionalPeers": ["peer-a"],
              "os": ["linux"]
            }, "sha512-LD"],
            "workspace-root": ["workspace:root"],
            "bad-shape": "not-an-array"
          }
        }"#,
    );

    let (lock, report) = import_file(&path).expect("test operation must succeed");
    // Only the valid npm pin became a package.
    // Chỉ pin npm hợp lệ thành package.
    assert_eq!(report.packages, 1);
    let lodash = &lock.packages[0];
    assert_eq!(lodash.name, "lodash");
    assert_eq!(lodash.version, "4.17.20");
    // Only production dependency names become edges; peer/platform keys do not.
    assert_eq!(
        lodash.dependencies,
        vec!["dep-a".to_string(), "dep-b".to_string()]
    );

    // Refused records are EXPLICIT — no silent data loss.
    // Record bị từ chối đều TƯỜNG MINH — không mất dữ liệu âm thầm.
    let skipped_keys: Vec<&str> = report.skipped.iter().map(|s| s.key.as_str()).collect();
    assert!(
        skipped_keys.contains(&"workspace-root"),
        "skipped: {skipped_keys:?}"
    );
    assert!(
        skipped_keys.contains(&"bad-shape"),
        "skipped: {skipped_keys:?}"
    );
    for s in &report.skipped {
        assert!(!s.reason.is_empty(), "every skip carries a reason");
    }
}

/// P0 finding #6: deno v5 workspace.dependencies are the ROOT GRAPH —
/// they land in lockfile.root_dependencies (root → dep edges), NEVER as
/// a self-edge on the resolved package ("lodash depends on lodash" was
/// the old lie). JSR pins survive (jsr: prefix) AND are listed as
/// skipped with the no-npm-mapping reason.
/// P0 finding #6: workspace.dependencies của deno v5 là GRAPH ROOT —
/// ghi vào lockfile.root_dependencies (cạnh root → dep), KHÔNG BAO GIỜ
/// là self-edge trên package được resolve ("lodash phụ thuộc lodash"
/// là câu sai cũ). Pin JSR sống sót (tiền tố jsr:) VÀ được liệt kê
/// skipped kèm lý do không-map-được-npm.
#[test]
fn deno_import_preserves_root_edges_and_lists_jsr_skips() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let path = write_lock(
        tmp.path(),
        "deno.lock",
        r#"{
  "version": "5",
  "specifiers": { "npm:lodash@4.17.20": "4.17.20" },
  "jsr": { "@std/bytes@0.224.0": { "integrity": "a225..." } },
  "npm": { "lodash@4.17.20": { "integrity": "sha512-LD" } },
  "workspace": { "dependencies": ["npm:lodash@4.17.20", "jsr:@std/bytes@0.224.0"] }
}"#,
    );

    let (lock, report) = import_file(&path).expect("test operation must succeed");
    assert_eq!(report.packages, 2); // lodash (npm) + jsr:@std/bytes

    let lodash = lock
        .packages
        .iter()
        .find(|p| p.name == "lodash")
        .expect("test operation must succeed");
    // Root pins are the ROOT GRAPH — the package itself must carry NO
    // self-edge (the old bug turned "root depends on lodash" into
    // "lodash depends on npm:lodash@4.17.20").
    // Pin root là GRAPH ROOT — chính package KHÔNG được có self-edge
    // (bug cũ biến "root phụ thuộc lodash" thành "lodash phụ thuộc
    // npm:lodash@4.17.20").
    assert!(
        lodash.dependencies.is_empty(),
        "self-edge found: {:?}",
        lodash.dependencies
    );
    assert_eq!(lodash.integrity, "sha512-LD");

    // The root graph lives in lockfile.root_dependencies, pins verbatim
    // (runtime prefixes preserved — no information loss).
    // Graph root nằm trong lockfile.root_dependencies, giữ nguyên pin
    // (tiền tố runtime được bảo toàn — không mất thông tin).
    assert_eq!(
        lock.root_dependencies,
        vec![
            "npm:lodash@4.17.20".to_string(),
            "jsr:@std/bytes@0.224.0".to_string()
        ]
    );

    let jsr = lock
        .packages
        .iter()
        .find(|p| p.name == "jsr:@std/bytes")
        .expect("test operation must succeed");
    assert_eq!(jsr.version, "0.224.0");

    // JSR mapped-but-unauditable is EXPLICITLY skipped with a reason.
    // JSR ánh-xạ-nhưng-không-audit-được bị skip TƯỜNG MINH kèm lý do.
    let jsr_skips: Vec<_> = report
        .skipped
        .iter()
        .filter(|s| s.key.starts_with("jsr:"))
        .collect();
    assert_eq!(jsr_skips.len(), 1, "jsr skips: {jsr_skips:?}");
    assert!(jsr_skips[0].reason.contains("no npm registry"));
}

/// P0-5 deterministic roundtrip: importing the SAME bun.lock twice
/// yields byte-identical package sets (order + fields) — no drift.
/// P0-5 roundtrip tất định: import CÙNG bun.lock hai lần cho tập
/// package giống hệt nhau (thứ tự + trường) — không trôi.
#[test]
fn bun_import_is_deterministic() {
    let payload = "{\n  \"packages\": {\n    \"b\": [\"b@2.0.0\", \"\", {\"a\": \"a@^1\"}, \"sha512-B\"],\n    \"a\": [\"a@1.0.0\", \"https://r/a.tgz\", {}, \"sha512-A\"],\n  }\n}";
    let tmp = TempDir::new().expect("test operation must succeed");
    let p1 = write_lock(tmp.path(), "bun.lock", payload);
    let (lock1, _) = import_file(&p1).expect("test operation must succeed");
    let tmp2 = TempDir::new().expect("test operation must succeed");
    let p2 = write_lock(tmp2.path(), "bun.lock", payload);
    let (lock2, _) = import_file(&p2).expect("test operation must succeed");
    assert_eq!(lock1.packages, lock2.packages);
}

// ------------------------------------------------ real-writer fixtures
// P0 finding #9 (2026-09-12): hand-written fixtures pass even when the
// importer drifts from what real writers emit. The fixtures below are
// generated by PINNED tools (bun 1.3.14 + 1.4.3, pnpm 12.12.0, deno 2.9.7) via
// scripts/gen-real-lockfile-fixtures.sh and committed; the tests
// assert the importer against REAL writer output — shape drift in
// either the tools or the importer breaks here, not in production.
// P0 finding #9: fixture viết tay pass ngay cả khi importer trôi khỏi
// output thật của writer. Fixture bên dưới do CÔNG CỤ GHIM (bun 1.3.14 + 1.4.3,
// pnpm 12.12.0, deno 2.9.7) sinh qua scripts/gen-real-lockfile-fixtures.sh
// và được commit; test đối chiếu importer với output writer THẬT —
// trôi shape ở tool hay importer đều vỡ ở đây, không vỡ ở production.

#[test]
fn real_bun_lockfile_imports() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/bun.lock");
    let (lock, report) = import_file(&path).expect("test operation must succeed");

    // The pinned packages (debug, ms, left-pad) must import with the REAL
    // integrities the writer recorded — verbatim, never synthesized.
    // Tất cả package ghim phải giữ nguyên integrity writer đã ghi.
    assert_eq!(report.packages, 3);
    let debug = lock
        .get_package("debug")
        .expect("test operation must succeed");
    assert_eq!(debug.version, "4.4.3");
    assert_eq!(debug.dependencies, vec!["ms"]);
    assert!(debug.integrity.starts_with("sha512-"));
    let ms = lock.get_package("ms").expect("test operation must succeed");
    assert_eq!(ms.version, "2.1.3");
    assert!(
        ms.integrity.starts_with("sha512-"),
        "real integrity missing: {}",
        ms.integrity
    );
    let left_pad = lock
        .get_package("left-pad")
        .expect("test operation must succeed");
    assert_eq!(left_pad.version, "1.3.0");
    assert!(left_pad.integrity.starts_with("sha512-"));
}

#[test]
fn real_bun_v1_lockfile_imports() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let fixture = include_str!("fixtures/bun-v1.lock.fixture");
    let path = write_lock(tmp.path(), "bun.lock", fixture);
    let (lock, report) = import_file(&path).expect("test operation must succeed");

    assert_eq!(report.packages, 2);
    assert_eq!(
        lock.get_package("ms")
            .expect("test operation must succeed")
            .version,
        "2.1.3"
    );
    assert_eq!(
        lock.get_package("left-pad")
            .expect("test operation must succeed")
            .version,
        "1.3.0"
    );
}

#[test]
fn real_pnpm_12_lockfile_imports_packages_and_integrity() {
    let tmp = TempDir::new().expect("test operation must succeed");
    let fixture = include_str!("fixtures/pnpm-lock.yaml.fixture");
    let path = write_lock(tmp.path(), "pnpm-lock.yaml", fixture);
    let (lock, report) = import_file(&path).expect("test operation must succeed");

    assert_eq!(report.packages, 2);
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
    let ms = lock.get_package("ms").expect("test operation must succeed");
    assert_eq!(ms.version, "2.1.3");
    assert_eq!(
        ms.integrity,
        "sha512-6FlzubTLZG3J2a/NVCAleEhjzq5oxgHyaCU9yYXvcLsvoVaHJq/s5xXI6/XXP6tz7R9xAOtHnSO/tXtF3WRTlA=="
    );
    let left_pad = lock
        .get_package("left-pad")
        .expect("test operation must succeed");
    assert_eq!(left_pad.version, "1.3.0");
    assert_eq!(
        left_pad.integrity,
        "sha512-XI5MPzVNApjAyhQzphX8BkmKsKUxD4LdyK24iZeQGinBN9yTQT3bFlCBy/aVx2HrNcqQGsdot8ghrjyrvMCoEA=="
    );
}

#[test]
fn real_deno_lockfile_imports_root_graph() {
    let path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/deno.lock");
    let (lock, report) = import_file(&path).expect("test operation must succeed");

    // npm pin imports with the REAL integrity; jsr pin survives with
    // its prefix and is listed as skipped (no npm advisory mapping).
    // Pin npm import với integrity THẬT; pin jsr sống sót với tiền tố
    // và được liệt kê skipped (không map được advisory npm).
    assert_eq!(report.packages, 2); // lodash (npm) + jsr:@std/bytes
    let lodash = lock
        .get_package("lodash")
        .expect("test operation must succeed");
    assert_eq!(lodash.version, "4.17.20");
    assert!(
        lodash.integrity.starts_with("sha512-"),
        "real integrity missing: {}",
        lodash.integrity
    );
    // Real writer shape: NO self-edge on the package (P0 finding #6) —
    // root pins live in lockfile.root_dependencies.
    // Shape writer thật: KHÔNG self-edge trên package (P0 finding #6)
    // — pin root nằm trong lockfile.root_dependencies.
    assert!(lodash.dependencies.is_empty());

    assert_eq!(
        lock.root_dependencies,
        vec![
            "jsr:@std/bytes@0.224.0".to_string(),
            "npm:lodash@4.17.20".to_string(),
        ],
        "root graph must carry both real pins verbatim"
    );

    let jsr = lock
        .packages
        .iter()
        .find(|p| p.name == "jsr:@std/bytes")
        .expect("jsr pin must survive with prefix");
    assert_eq!(jsr.version, "0.224.0");

    let jsr_skips: Vec<_> = report
        .skipped
        .iter()
        .filter(|s| s.key.starts_with("jsr:"))
        .collect();
    assert_eq!(jsr_skips.len(), 1, "jsr skips: {jsr_skips:?}");
}
