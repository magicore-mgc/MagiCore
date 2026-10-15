//! Centralized CLI error messages (English only — RULE §7).
//! Mọi error message của CLI định nghĩa tập trung tại đây.

use anyhow::{Error, anyhow};
use std::path::Path;

/// A v4 lock cannot be signed while a legacy detached signature is present.
/// Không ký lock v4 khi sidecar chữ ký legacy vẫn tồn tại.
pub fn lock_v4_legacy_signature_sidecar() -> Error {
    anyhow!(
        "v4 lockfile cannot use a legacy .lock.sig sidecar; remove or explicitly migrate the stale sidecar before signing"
    )
}

/// Refuse to re-sign a v4 document whose existing integrity evidence fails.
/// Từ chối ký lại v4 nếu bằng chứng toàn vẹn hiện hữu không hợp lệ.
pub fn lock_v4_existing_signature_invalid() -> Error {
    anyhow!("refusing to re-sign a v4 lockfile whose existing digest or signature is invalid")
}

pub fn trusted_registry_admin_required() -> Error {
    anyhow!("trusted publisher management requires --admin-token or MAGICORE_REGISTRY_ADMIN_TOKEN")
}

pub fn trusted_binding_failed(status: u16) -> Error {
    anyhow!("trusted publisher binding request failed with HTTP {status}")
}

pub fn trusted_registry_url_invalid() -> Error {
    anyhow!("trusted publishing requires a valid registry URL without credentials or fragment")
}

pub fn trusted_registry_requires_https() -> Error {
    anyhow!("trusted publishing requires HTTPS except for loopback registry URLs")
}

pub fn trusted_audience_required() -> Error {
    anyhow!("trusted publishing audience cannot be empty")
}

pub fn trusted_token_exchange_failed(status: u16) -> Error {
    anyhow!("trusted publishing token exchange failed with HTTP {status}")
}

pub fn trusted_oci_upload_start_failed(status: u16) -> Error {
    anyhow!("OCI blob upload start returned HTTP {status} instead of 202")
}

pub fn trusted_oci_upload_chunk_failed(status: u16) -> Error {
    anyhow!("OCI blob upload chunk returned HTTP {status} instead of 202")
}

pub fn trusted_oci_upload_complete_failed(status: u16) -> Error {
    anyhow!("OCI blob upload completion returned HTTP {status} instead of 201")
}

pub fn trusted_oci_upload_location_missing() -> Error {
    anyhow!("OCI registry omitted a required upload Location header")
}

pub fn trusted_oci_upload_range_invalid() -> Error {
    anyhow!("OCI registry returned an unexpected upload Range header")
}

pub fn trusted_token_response_invalid() -> Error {
    anyhow!("registry returned an invalid trusted publishing token response")
}

pub fn audit_signatures_flags_conflict() -> Error {
    anyhow!("audit signatures cannot be combined with vulnerability audit flags")
}

pub fn trusted_signatures_request_failed(status: u16) -> Error {
    anyhow!("trusted signatures request failed with HTTP {status}")
}

pub fn trusted_signatures_response_too_large() -> Error {
    anyhow!("trusted signatures response exceeds the configured size limit")
}

pub fn trusted_signatures_response_invalid() -> Error {
    anyhow!("registry returned an invalid trusted signatures response")
}

pub fn trusted_signatures_missing_or_mismatched() -> Error {
    anyhow!("registry returned no provenance entries or a package mismatch")
}

pub fn trusted_signatures_invalid() -> Error {
    anyhow!("trusted provenance signature or transparency chain verification failed")
}

/// Keep project dependency installs separate from persistent user CLI tools.
/// Tách dependency của project khỏi CLI tool bền vững theo user.
pub fn global_package_install_unsupported() -> Error {
    anyhow!(
        "`mgc add --global` is not supported: `mgc add` installs in the current project. Use `mgc tool install npm:<package>` for a persistent npm CLI or `mgc dlx <package>` for temporary execution; no project changes were made"
    )
}

/// A core without a preview implementation must not execute an install as a dry run.
/// Core chưa có preview phải từ chối trước khi chạy install với cờ dry-run.
pub fn install_dry_run_unsupported(core: &str) -> Error {
    anyhow!("dry-run is not supported for core '{core}'; no changes were made")
}

/// An optimizer environment value has an unmatched outer quote.
/// Giá trị env của optimizer có quote ngoài không cân bằng.
pub fn optimizer_env_unmatched_quote(line: usize) -> Error {
    anyhow!("optimizer env file has unmatched quote at line {line}")
}

/// Optimizer configuration must be a normal directory and file tree.
/// Cấu hình optimizer chỉ được đọc từ thư mục và file thường.
pub fn optimizer_config_directory_invalid(path: &Path) -> Error {
    anyhow!(
        "optimizer configuration directory '{}' must be a regular directory, not a symlink or special file",
        path.display()
    )
}

/// An optimizer Cargo setting is outside the consumed, validated schema.
/// Setting Cargo của optimizer nằm ngoài schema được kiểm tra và sử dụng.
pub fn optimizer_config_setting_invalid(path: &Path, setting: &str) -> Error {
    anyhow!(
        "optimizer Cargo configuration '{}' has an unsupported or invalid setting '{setting}'",
        path.display()
    )
}

/// Optimizer config reads must not follow a link or accept special files.
/// Đọc cấu hình optimizer không theo symlink và không nhận file đặc biệt.
pub fn optimizer_config_read_failed(path: &Path) -> Error {
    anyhow!(
        "could not safely read optimizer configuration '{}'",
        path.display()
    )
}

/// An optimizer configuration disappeared between discovery and reading.
/// Cấu hình optimizer biến mất giữa lúc phát hiện và đọc.
pub fn optimizer_config_missing(path: &Path) -> Error {
    anyhow!(
        "optimizer configuration '{}' disappeared while being read",
        path.display()
    )
}

/// `mgc remove-ai <pkg> [pkg...]` — name packages to remove
pub fn remove_ai_usage() -> Error {
    anyhow!("mgc remove-ai <pkg> [pkg...] — name the packages to remove")
}

/// `mgc remove-app <pkg> [pkg...]` — name packages to remove
pub fn remove_app_usage() -> Error {
    anyhow!("mgc remove-app <pkg> [pkg...] — name the packages to remove")
}

/// `mgc add-app <pkg> [pkg...]` — name packages to add
pub fn add_app_usage() -> Error {
    anyhow!("mgc add-app <pkg> [pkg...] — name the packages to add")
}

/// verb not applicable to cicd core (07 §4)
pub fn cicd_verb_not_applicable(verb: &str) -> Error {
    anyhow!(
        "'{verb}' does not apply to the cicd core (07 §4) — use `mgc ci generate`, `mgc verify`, or `mgc deploy`."
    )
}

/// templates/web missing on disk
pub fn web_templates_missing(root: &std::path::Path) -> Error {
    anyhow!("templates/web does not exist on disk: {}", root.display())
}

/// no web layer found with template.toml+sources
pub fn no_web_layer_found(dir: &std::path::Path) -> Error {
    anyhow!(
        "No web layer with template.toml+sources found in {}",
        dir.display()
    )
}

/// template publish required before fetch
pub fn template_not_published(url: &str, http: u16) -> Error {
    anyhow!(
        "Template '{}' not found at {} (HTTP {}) — run `mgc template publish` first",
        url,
        url,
        http
    )
}

/// HF request failed
pub fn hf_request_failed() -> Error {
    anyhow!("HF request failed")
}

/// pull manifest failed — registry running?
pub fn pull_manifest_failed() -> Error {
    anyhow!("pull manifest failed (is the registry running? `mgc registry serve`)")
}

/// parse manifest failed
pub fn parse_manifest_failed() -> Error {
    anyhow!("parse manifest failed")
}

/// file does not exist
pub fn file_not_found(path: &std::path::Path) -> Error {
    anyhow!("file does not exist: {}", path.display())
}

/// path does not exist
pub fn path_not_found(p: &std::path::Path) -> Error {
    anyhow!("path does not exist: {}", p.display())
}

/// Patch metadata contains a path outside the project-owned patch store.
pub fn patch_path_rejected(reason: &str) -> Error {
    anyhow!("patch verification refused the configured patch path: {reason}")
}

/// A package/version-range patch identity already exists in the project.
pub fn patch_duplicate_identity() -> Error {
    anyhow!(
        "a patch already exists for this package and version range; remove it before adding a replacement"
    )
}

/// directory has no files
pub fn dir_has_no_files(p: &std::path::Path) -> Error {
    anyhow!("directory has no files: {}", p.display())
}

/// push model failed — server running?
pub fn push_model_failed() -> Error {
    anyhow!("push model failed — is the server running? (`mgc registry serve`)")
}

/// catalog listing failed
pub fn catalog_failed() -> Error {
    anyhow!("catalog listing failed")
}

/// token not found (revoked?)
pub fn token_not_found(token: &str) -> Error {
    anyhow!("Token {} not found (was it revoked?)", token)
}

/// web project missing package.json — cannot run tests
pub fn web_missing_package_json() -> Error {
    anyhow!("web project is missing package.json — cannot run tests")
}

/// package.json missing scripts.test
pub fn package_json_missing_test_script() -> Error {
    anyhow!("package.json is missing scripts.test")
}

/// lib core has no test runner for this language yet
pub fn lib_no_test_runner() -> Error {
    anyhow!("lib core has no test runner for this language yet (07 §4 P1)")
}

/// CICD verification test step failed for a specific core.
/// Bước test trong CICD verify thất bại cho một core cụ thể.
pub fn cicd_test_step_failed(core: &str, reason: &dyn std::fmt::Display) -> Error {
    anyhow!("test step for core '{core}' failed: {reason}")
}

/// deploy target missing `stack` in [deploy] targets
pub fn deploy_target_missing_stack() -> Error {
    anyhow!("aws target is missing `stack` in [deploy] targets — example: stack = \"my-infra\"")
}

/// deploy target adapter unknown
pub fn deploy_target_unknown(other: &str) -> Error {
    anyhow!("deploy target '{other}' has no adapter — supported: aws | cloudflare | gcp")
}

/// no deploy targets configured
pub fn no_deploy_targets() -> Error {
    anyhow!("no deploy target — add [deploy] targets to mgc.toml")
}

/// python -m build failed
pub fn python_build_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!(
        "Python build backend failed: {e} — MagiCore does not install Python build tools; provide a supported build backend before retrying"
    )
}

/// go build ./... failed
pub fn go_build_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!("go build ./... failed: {e}")
}

/// dotnet build failed
pub fn dotnet_build_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!("dotnet build failed: {e} — install the .NET SDK first")
}

/// Android device discovery or launch failed.
/// Không thể truy vấn hoặc khởi chạy ứng dụng trên thiết bị Android.
pub fn android_device_command_failed(e: impl std::fmt::Display) -> Error {
    anyhow!("Android device command failed: {e}")
}

/// flash only supports esp32-rust currently
pub fn flash_framework_unsupported(framework: &str) -> Error {
    anyhow!(
        "'mgc flash' currently supports only the esp32-rust framework (you are using {framework}) — platformio/zephyr flash is P2"
    )
}

/// cloudflare workers build/deploy belongs to cicd core
pub fn cloudflare_build_in_cicd_core() -> Error {
    anyhow!(
        "cloudflare workers build/deploy belongs to the cicd core (Q12) — cloud core does not handle it as P1"
    )
}

/// unknown hardware package for add-hardware
pub fn unknown_hardware_package(pkg: &str) -> Error {
    anyhow!("unknown hardware package '{pkg}' (optimizer | bench)")
}

/// Hardware template directory is absent — nothing to remove.
/// (Thư mục template hardware không có — không có gì để gỡ.)
pub fn hardware_template_not_materialized(pkg: &str, root: &std::path::Path) -> Error {
    anyhow!(
        "hardware template '{pkg}' is not materialized in '{}' — nothing to remove",
        root.display()
    )
}

/// Hardware template directory could not be removed.
/// (Không gỡ được thư mục template hardware.)
pub fn hardware_template_remove_failed(pkg: &str, reason: &dyn std::fmt::Display) -> Error {
    anyhow!("cannot remove hardware template '{pkg}': {reason}")
}

/// `mgc trust pending` could not open the machine-local policy database.
/// `mgc trust pending` không mở được database policy cục bộ.
pub fn trust_policy_database_open_failed(path: &std::path::Path) -> Error {
    anyhow!(
        "failed to open trust policy database '{}': database access failed",
        path.display()
    )
}

/// `mgc trust pending` could not read the machine-local policy database.
/// `mgc trust pending` không đọc được policy từ database cục bộ.
pub fn trust_policy_database_read_failed(path: &std::path::Path) -> Error {
    anyhow!(
        "failed to read trust policies from '{}': database query failed",
        path.display()
    )
}

/// A trust-policy write or prune could not update its database.
/// Không thể ghi hoặc dọn database trust policy.
pub fn trust_policy_database_write_failed(
    path: &std::path::Path,
    reason: &dyn std::fmt::Display,
) -> Error {
    anyhow!(
        "failed to update trust policy database '{}': {reason}",
        path.display()
    )
}

/// A trust-policy mutation could not obtain the project writer lock.
/// Không lấy được khóa ghi project để sửa trust policy.
pub fn trust_policy_lock_failed(root: &std::path::Path, reason: &dyn std::fmt::Display) -> Error {
    anyhow!(
        "cannot update trust policy in '{}' while project state is changing or unrecovered: {reason}",
        root.display()
    )
}

/// A package identifier is required for trust approval or denial.
/// Cần package identifier để approve hoặc deny trust.
pub fn trust_policy_package_empty() -> Error {
    anyhow!("package name cannot be empty")
}

/// `mgc trust anchor` found no core marker to attest.
/// (Không có core marker để chứng thực.)
pub fn trust_anchor_no_marker(root: &std::path::Path) -> Error {
    anyhow!(
        "no core marker in '{}' — run `mgc init` first, then `mgc trust anchor`",
        root.display()
    )
}

/// `mgc trust anchor` cannot access the home directory for key storage.
/// (Không truy cập được home để lưu key.)
pub fn trust_anchor_no_home(reason: &dyn std::fmt::Display) -> Error {
    anyhow!("cannot access global identities dir: {reason}")
}

/// `mgc trust anchor` failed to attest the project core.
/// (Chứng thực core thất bại.)
pub fn trust_anchor_failed(reason: &dyn std::fmt::Display) -> Error {
    anyhow!("core attestation failed: {reason}")
}

/// `--rotate` generates a fresh key by definition; naming a key with it
/// is contradictory — use plain `mgc trust anchor --key-id` to switch.
/// (`--rotate` sinh key mới theo định nghĩa; chỉ định key cùng lúc là
/// mâu thuẫn.)
pub fn trust_anchor_rotate_needs_generated_key() -> Error {
    anyhow!(
        "--rotate generates a fresh key and cannot be combined with --key-id (use --key-id alone to switch keys)"
    )
}

/// A trust-policy storage path is not a regular project-owned path.
/// Đường dẫn lưu trust policy không thuộc cây thư mục project an toàn.
pub fn trust_policy_path_invalid(path: &std::path::Path) -> Error {
    anyhow!(
        "refusing unsafe trust policy storage path '{}' (symlinks and non-regular paths are not allowed)",
        path.display()
    )
}

/// A project TOML update could not obtain the shared project writer lock.
/// Không lấy được khóa project để cập nhật mgc.toml.
pub fn config_project_lock_failed(root: &std::path::Path, reason: &dyn std::fmt::Display) -> Error {
    anyhow!(
        "cannot update project configuration in '{}' while project state is changing or unrecovered: {reason}",
        root.display()
    )
}

/// `mgc trust pending` found an unreadable or malformed committed script policy.
/// `mgc trust pending` gặp policy script commit bị lỗi hoặc sai định dạng.
pub fn trust_scripts_policy_invalid(reason: &dyn std::fmt::Display) -> Error {
    anyhow!("failed to load [scripts] trust policy: {reason}")
}

/// `mgc trust pending` could not completely inspect installed packages.
/// `mgc trust pending` không thể quét đầy đủ dữ liệu package đã cài.
pub fn trust_pending_scan_failed(path: &std::path::Path, reason: &dyn std::fmt::Display) -> Error {
    anyhow!(
        "failed to inspect lifecycle-script package data at '{}': {reason}",
        path.display()
    )
}

/// An installed package manifest is valid JSON but has an invalid trust-relevant shape.
/// Manifest đã cài có JSON hợp lệ nhưng sai cấu trúc liên quan đến trust.
pub fn trust_pending_manifest_invalid(path: &std::path::Path, reason: &str) -> Error {
    anyhow!(
        "invalid lifecycle-script package manifest '{}': {reason}",
        path.display()
    )
}

/// An installed package manifest exceeds the bounded trust scan size.
/// Manifest package cài đặt vượt giới hạn kích thước khi quét trust.
pub fn trust_pending_manifest_too_large(path: &std::path::Path, limit: u64) -> Error {
    anyhow!(
        "lifecycle-script package manifest '{}' exceeds the {limit}-byte inspection limit",
        path.display()
    )
}

/// The trust-pending scan could not obtain a stable project snapshot.
/// Không lấy được snapshot project ổn định khi quét trust pending.
pub fn trust_pending_lock_failed(root: &std::path::Path, reason: &dyn std::fmt::Display) -> Error {
    anyhow!(
        "cannot scan trust-pending packages in '{}' while project state is changing or unrecovered: {reason}",
        root.display()
    )
}

// ===== chung (nhiều file dùng) =====

/// cwd đã bị xóa khi resolve
pub fn cwd_deleted(e: &std::io::Error) -> Error {
    anyhow!("failed to resolve current working directory — has it been deleted?: {e}")
}

/// không detect được project theo core kind
pub fn no_mgc_project_found(kind: &str) -> Error {
    let msg = match kind {
        "game" => {
            "No MagiCore game project found (missing mgc.toml with ecosystem = \"game\" \
                   or project.godot/Packages/manifest.json/.uproject/Cargo.toml in the current project)"
        }
        "iot" => {
            "No MagiCore IoT project found (missing mgc.toml with ecosystem = \"iot\" \
                  or platformio.ini/west.yml/Cargo.toml in the current project)"
        }
        "lib" | "library" => {
            "No MagiCore library project found (missing mgc.toml with \
                  ecosystem = \"lib\" or Cargo.toml/package.json/pyproject.toml in the current project)"
        }
        "clo" | "cloud" => {
            "No MagiCore cloud project found (missing mgc.toml with ecosystem = \"cloud\" \
                  or Pulumi.yaml/*.tf/cdk package.json in the current project)"
        }
        "app" => {
            "No MagiCore app project found (missing mgc.toml with ecosystem = \"app\" \
                  or pubspec.yaml/build.gradle/.kts/Package.swift in the current project)"
        }
        "web" => {
            "No MagiCore project found (current directory and its parents contain no mgc.toml, mgc.lock, package.json, Cargo.toml, pyproject.toml, or .git)"
        }
        _ => {
            "No MagiCore project found (missing mgc.toml or known project manifest in the current directory tree)"
        }
    };
    anyhow!(msg)
}

/// router core không biết core
pub fn unknown_core(core: &str) -> Error {
    anyhow!("Unknown core '{core}'")
}

/// bare package command needs an explicit or detected core
pub fn bare_core_not_detected(verb: &str) -> Error {
    anyhow!(
        "`mgc {verb}` could not detect a MagiCore core in this project. Use `mgc --core <core> {verb}` or the explicit command such as `mgc {verb}-web`."
    )
}

/// chưa chạy mgc init
pub fn project_root_missing() -> Error {
    anyhow!("Project root not found — run mgc init")
}

/// thiếu mgc.toml
pub fn mgc_toml_missing() -> Error {
    anyhow!("mgc.toml not found — run mgc init")
}

/// không detect framework theo loại
pub fn no_framework_detected(framework: &str, root: &std::path::Path) -> Error {
    anyhow!("No {framework} framework detected in {}", root.display())
}

/// tool chạy fail (generic)
pub fn tool_failed(tool: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!("{tool} failed: {e}")
}

/// vượt MAX_PACKAGES
pub fn too_many_packages(count: usize, verb: &str) -> Error {
    anyhow!("Too many packages ({count}). Maximum per {verb} command is 50.")
}

/// lockfile mới hơn mgc hỗ trợ
pub fn lockfile_newer(version: u32, supported: u32) -> Error {
    anyhow!(
        "mgc.lock version {version} is newer than this version of mgc (supports up to {supported}). \
         Upgrade mgc to read this lockfile."
    )
}

/// dependency id trong lockfile không parse được
pub fn invalid_dep_id(dep: &str, name: &str, version: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!("invalid dependency id '{dep}' in lockfile package '{name}@{version}': {e}")
}

/// package không thấy local/global
pub fn pkg_not_found_local(package: &str) -> Error {
    anyhow!(
        "package '{package}' not found in local node_modules or MagiCore global package roots.\n\
         Use 'mgc install' to install it first, provide a local path, or configure MAGICORE_GLOBAL_PACKAGE_ROOT."
    )
}

// ===== install/app.rs =====

pub fn app_not_available(reason: &str) -> Error {
    anyhow!("'app' {reason}")
}

pub fn app_project_not_detected() -> Error {
    anyhow!(
        "Cannot detect an app project here (missing mgc.toml [app] language / pubspec.yaml / build.gradle/.kts / Package.swift)."
    )
}

pub fn no_app_language(root: &std::path::Path) -> Error {
    anyhow!("No app language detected in {}", root.display())
}

pub fn app_tool_failed(cmd: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!("{cmd} failed: {e} — install the tool first and ensure it is in PATH")
}

/// verb không có CLI passthrough cho language
pub fn manifest_hint(verb: &str, lang: &str, file: &str) -> Error {
    anyhow!("'{verb}' for {lang:?} has no CLI passthrough — edit {file} then run `mgc install`.")
}

pub fn xcode_project_missing(root: &std::path::Path) -> Error {
    anyhow!(
        "no *.xcworkspace found/*.xcodeproj under {} — objC app missing an Xcode project",
        root.display()
    )
}

pub fn xcode_project_missing_short() -> Error {
    anyhow!("no *.xcworkspace found/*.xcodeproj — objC app missing an Xcode project")
}

pub fn objc_dev_needs_xcode(proj: &str) -> Error {
    anyhow!(
        "objC dev runs in Xcode — open {proj}, or set [app] dev_scheme = \"<scheme>\" in mgc.toml so `mgc dev` runs xcodebuild build on the simulator"
    )
}

// ===== install/ai.rs =====

pub fn ai_no_lockfile() -> Error {
    anyhow!(
        "no lock file (uv.lock or requirements.lock) — run `mgc add <pkg>` to create a lock first"
    )
}

// ===== shared.rs =====

pub fn frozen_lock_missing(cmd: &str) -> Error {
    anyhow!(
        "--frozen: mgc.lock is missing or does not match package.json.\n\
         Run '{cmd}' to generate an up-to-date lockfile."
    )
}

/// Frozen install with a PRESENT but mismatching lockfile — a possible
/// tamper, never silently re-resolved.
pub fn frozen_lock_mismatch(cmd: &str) -> Error {
    anyhow!(
        "--frozen: mgc.lock exists but does not match the manifest (drift or tamper) — refusing to re-resolve.\n\
         Run '{cmd}' without --frozen to regenerate after reviewing the diff."
    )
}

pub fn audit_strict_web_only(name: &str) -> Error {
    anyhow!(
        "--audit-strict is only implemented for the web core right now; refusing to claim policy parity for '{name}'"
    )
}

/// `--audit-strict` was passed to a command that never consults strictness.
/// Accepting it would silently ignore the flag — refuse with the supported set.
pub fn audit_strict_unsupported_command(name: &str) -> Error {
    anyhow!(
        "--audit-strict has no effect on '{name}' (strict audit applies to: audit, verify, install, add and their per-core variants)"
    )
}

pub fn audit_strict_no_web_adapter() -> Error {
    anyhow!(
        "--audit-strict requires the web core registry adapter, which is not included in this build"
    )
}

pub fn audit_parse_time_failed(name: &str, version: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!("failed to parse published time for '{name}@{version}': {e}")
}

pub fn audit_recent_blocked(name: &str, version: &str, published_at: &str) -> Error {
    anyhow!(
        "--audit-strict blocked '{name}@{version}' because it was published within the last 24 hours ({published_at})"
    )
}

pub fn audit_api_status(status: &reqwest::StatusCode) -> Error {
    anyhow!("--audit-strict advisory API returned {status}")
}

pub fn audit_advisory_blocked(pkg: &str, severity: &str, title: &str) -> Error {
    anyhow!("--audit-strict blocked '{pkg}' due to {severity} advisory: {title}")
}

pub fn link_web_only() -> Error {
    anyhow!("link only supported for web core")
}

pub fn link_usage() -> Error {
    anyhow!("Usage: mgc link <package>")
}

pub fn link_exists(name: &str) -> Error {
    anyhow!("{name} already exists in node_modules")
}

pub fn unlink_web_only() -> Error {
    anyhow!("unlink only supported for web core")
}

pub fn unlink_usage() -> Error {
    anyhow!("Usage: mgc unlink <package>")
}

pub fn unlink_not_linked(name: &str) -> Error {
    anyhow!("{name} is not linked in node_modules")
}

pub fn why_web_only() -> Error {
    anyhow!("why only supported for web core")
}

pub fn lock_missing_install() -> Error {
    anyhow!("mgc.lock not found — run 'mgc install' first")
}

// P0-4 (2026-09-15): `mgc why` used to `unimplemented!()` (panic) when it
// reached the v1 lockfile path. A user-reachable panic is never acceptable:
// the command now returns a typed error that explains the lockfile v2
// migration instead of aborting the process.
// (P0-4: `mgc why` từng `unimplemented!()` (panic) khi đi tới nhánh
// lockfile v1. Panic chạm tới được từ user là không chấp nhận được: lệnh
// giờ trả typed error giải thích migration lockfile v2 thay vì abort.)
pub fn why_requires_lockfile_v2() -> Error {
    anyhow!(
        "`mgc why` requires the lockfile v2 graph (dependency-reason lookup is not \
         available in the legacy v1 mgc.lock schema). Re-run `mgc install` to \
         regenerate mgc.lock in the v2 format; until that migration completes, \
         `mgc why` stays unavailable for lockfiles written by the v1 installer."
    )
}

/// `mgc why` target is absent from mgc.lock — nothing can depend on it.
/// (Package hỏi `why` không có trong lock — không gì phụ thuộc nó.)
pub fn why_package_not_in_lock(package: &str) -> Error {
    anyhow!(
        "`mgc why {package}`: '{package}' is not pinned in mgc.lock — nothing in this project can depend on it."
    )
}

pub fn local_path_not_found(path: &std::path::Path) -> Error {
    anyhow!("local package path not found: {}", path.display())
}

pub fn cargo_toml_no_deps() -> Error {
    anyhow!("root Cargo.toml missing [dependencies]")
}

pub fn ai_project_not_detected() -> Error {
    anyhow!(
        "Cannot detect an ai project here (missing mgc.toml [ai] framework / pyproject [tool.magicore] framework)."
    )
}

pub fn no_ai_framework(root: &std::path::Path) -> Error {
    anyhow!("No ai framework detected in {}", root.display())
}

pub fn python3_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!("python3 failed: {e} — install Python 3.11+ and ensure `python3` is in PATH")
}

/// Python launcher failed — Python launcher đã thất bại.
pub fn python_runtime_failed(launcher: &str, error: &dyn std::fmt::Display) -> Error {
    anyhow!("Python launcher '{launcher}' failed: {error}")
}

/// Native Python path cannot be represented in the child environment.
/// Không biểu diễn được native Python path trong môi trường tiến trình con.
pub fn python_path_not_unicode() -> Error {
    anyhow!(
        "native Python dependency path is not valid Unicode and cannot be passed through this command environment"
    )
}

// ===== dev.rs =====

pub fn path_not_utf8() -> Error {
    anyhow!("project path is not valid UTF-8")
}

pub fn unreal_editor_dev(uproject: &str) -> Error {
    anyhow!(
        "unreal dev runs in Unreal Editor (engine binary is named by version, no standard PATH) — open {uproject} in the editor, or install UnrealBuildTool and run the build directly. Run `mgc build` + the editor to run."
    )
}

pub fn dev_no_command_for_engine(engine: &str) -> Error {
    anyhow!("'mgc dev' has no command for engine '{engine}' — use the engine editor to run it")
}

pub fn godot_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!(
        "godot failed: {e} — install the Godot editor first (https://godotengine.org) and ensure `godot` is in PATH"
    )
}

pub fn espflash_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!(
        "espflash failed: {e} — install espflash first (`cargo install espflash`) and ensure it is in PATH"
    )
}

// ===== dev/cicd.rs =====

pub fn cicd_reason(reason: &str) -> Error {
    anyhow!("'cicd' {reason}")
}

pub fn ci_template_unknown(provider: &str) -> Error {
    anyhow!(
        "'mgc ci generate' has no template for provider '{provider}' — supported: github-actions | gitlab | circleci"
    )
}

pub fn cicd_project_not_detected() -> Error {
    anyhow!(
        "Cannot detect a cicd project here (missing mgc.toml [cicd] provider / wrangler.toml / argocd / .github/workflows)."
    )
}

pub fn cicd_verify_unknown_step(step: &str) -> Error {
    anyhow!(
        "mgc.toml [cicd] verify contains unknown step '{step}' — allowed steps: audit, test, build. Fix the config; verify must never skip unknown steps silently."
    )
}

pub fn cicd_verify_empty_chain() -> Error {
    anyhow!(
        "mgc.toml [cicd] verify chain is empty — a verify run with zero steps cannot prove anything. Add at least one of: audit, test, build."
    )
}

// ===== dev/clo.rs =====

pub fn deploy_not_implemented(cloud: &str) -> Error {
    anyhow!(
        "'mgc deploy' for '{cloud}' has no MagiCore-native provider engine yet; no external provider CLI was started"
    )
}

pub fn cicd_deploy_target_not_implemented(provider: &str, stack: &str, region: &str) -> Error {
    anyhow!(
        "'mgc deploy' for {provider} target '{stack}' in region '{region}' has no MagiCore-native provider engine yet; no external provider CLI was started"
    )
}

pub fn tool_not_installed_project(tool: &str) -> Error {
    anyhow!(
        "{tool} is not installed in the project — run `mgc install` first (tools resolve from node_modules/.bin/{tool})"
    )
}

pub fn project_tool_failed(tool: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!(
        "{tool} failed: {e} — run `mgc install` in this project first (tools install via node_modules/.bin)"
    )
}

// ===== dev/iot.rs =====

pub fn no_board_specified(boards: &str) -> Error {
    anyhow!(
        "No board specified — add `[iot] board = \"<id>\"` to mgc.toml (known boards: {boards}) or pass `mgc flash --board <id>`"
    )
}

pub fn unsupported_board(board: &str, boards: &str) -> Error {
    anyhow!("Unsupported board '{board}' — known boards: {boards}")
}

pub fn fw_path_not_utf8(elf: &std::path::Path) -> Error {
    anyhow!("firmware path is not valid UTF-8: {}", elf.display())
}

// ===== dev/app.rs =====

pub fn multi_dev_flutter_only() -> Error {
    anyhow!("multi dev P1 targets flutter/ entry — run `mgc build` for other platforms")
}

// ===== create/hardware.rs =====

pub fn unknown_hardware_framework(fw: &str) -> Error {
    anyhow!(
        "unknown hardware framework '{fw}' (optimizer | bench) — templates materialize directly into the project; no separate project scaffolding"
    )
}

pub fn no_hardware_framework() -> Error {
    anyhow!("no hardware framework selected")
}

// ===== build.rs =====

pub fn no_project_found_build() -> Error {
    anyhow!("No project found (no mgc.toml, package.json, or Cargo.toml)")
}

pub fn join_paths(err: &std::env::JoinPathsError) -> Error {
    anyhow!("{err}")
}

// ===== model/mod.rs =====

pub fn invalid_oci_source(oci: &str) -> Error {
    anyhow!("invalid oci source '{oci}' — use `oci://registry/repo:tag`")
}

pub fn invalid_model_name(name: &str) -> Error {
    anyhow!("model name must be a safe relative path: '{name}'")
}

pub fn unsafe_model_manifest_path() -> Error {
    anyhow!("model manifest path contains a symlink or non-directory component")
}

pub fn hf_revision_required() -> Error {
    anyhow!("hf:// pulls require --revision with an immutable 40-hex commit SHA")
}

pub fn invalid_hf_revision() -> Error {
    anyhow!("HF revision must be an immutable 40-hex commit SHA")
}

pub fn hf_sha256_required() -> Error {
    anyhow!("hf:// pulls require --sha256 with the trusted 64-hex artifact digest")
}

pub fn invalid_hf_sha256() -> Error {
    anyhow!("HF SHA-256 must contain exactly 64 hexadecimal characters")
}

pub fn invalid_hf_source() -> Error {
    anyhow!("invalid hf source; expected hf://<org>/<model>/<file>")
}

pub fn hf_size_overflow() -> Error {
    anyhow!("HF artifact size overflow")
}

pub fn hf_download_limit_exceeded(limit: u64) -> Error {
    anyhow!("HF artifact exceeded --max-bytes ({limit}); download was not imported into CAS")
}

pub fn hf_content_length_mismatch(expected: u64, actual: u64) -> Error {
    anyhow!("HF artifact Content-Length mismatch: expected {expected}, received {actual}")
}

pub fn hf_sha256_mismatch(expected: &str, actual: &str) -> Error {
    anyhow!("HF artifact SHA-256 mismatch: expected {expected}, got {actual}")
}

pub fn hf_artifact_changed_during_import() -> Error {
    anyhow!("HF artifact changed while being imported into CAS")
}

pub fn model_pull_flags_require_hf() -> Error {
    anyhow!("--revision, --sha256, and --max-bytes apply only to hf:// model sources")
}

pub fn unsupported_quantize_target(target: &str) -> Error {
    anyhow!("unsupported target: {target} (use q4_k_m or q8_0; awq needs a GPU toolchain)")
}

pub fn llama_cpp_missing() -> Error {
    anyhow!(
        "llama_cpp is unavailable in the MagiCore-managed environment; this model operation is unsupported until MagiCore can resolve and install the required native artifact"
    )
}

pub fn llama_quantize_failed(code: Option<i32>) -> Error {
    anyhow!("llama_cpp.quantize fail (exit {code:?})")
}

// ===== audit =====

/// Exit-code marker for audit environment/tool failures.
/// `std::process::exit` contract (Tech Lead §1): 0 = clean, 1 = findings
/// or policy violation, 2 = the audit could NOT run (tool missing,
/// environment broken). Carried through anyhow via downcast in main.
/// Marker exit-code cho lỗi môi trường/tool của audit: 0 = sạch, 1 = có
/// finding/policy vi phạm, 2 = audit KHÔNG chạy được (thiếu tool, hỏng
/// môi trường) — main downcast để lấy đúng mã.
#[derive(Debug)]
pub struct AuditExitError {
    pub exit_code: i32,
    pub message: String,
}

impl AuditExitError {
    /// Environment failure — scanner unavailable in strict mode.
    /// Lỗi môi trường — scanner unavailable ở strict mode.
    pub fn environment(reason: &str) -> Self {
        Self {
            exit_code: 2,
            message: format!(
                "Audit failed: scanner unavailable in strict mode ({reason})\n\
                 Set MGC_AUDIT_STRICT=0 to allow the unverified state locally (not recommended in CI)"
            ),
        }
    }
}

impl std::fmt::Display for AuditExitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AuditExitError {}

pub fn audit_fix_web_only() -> Error {
    anyhow!("audit --fix is only supported for the web core")
}

pub fn audit_unknown_scanner_state(core: &str) -> anyhow::Error {
    anyhow!(
        "audit scanner status for '{core}' could not be classified — refusing to interpret the report"
    )
}

/// Environment/tooling failure (scanner unavailable under strict mode).
/// Distinct exit code 2 so CI can tell "vulnerabilities found" (1) from
/// "the audit could not run" (2) — Tech Lead contract §1.
/// Lỗi môi trường/tool (scanner unavailable ở strict mode). Exit code 2
/// riêng để CI phân biệt "có lỗ hổng" (1) với "audit không chạy được" (2).
pub fn audit_scanner_unavailable_strict(reason: &str) -> Error {
    Error::msg(AuditExitError::environment(reason))
}

pub fn audit_found_vulnerabilities(count: usize, packages: usize) -> Error {
    anyhow!("audit found {count} vulnerabilities across {packages} packages")
}

/// Unknown `--format` value for `mgc audit` (P1-B): a hard usage error —
/// never a silent fallback that would surprise CI ingest.
/// Giá trị `--format` lạ cho `mgc audit`: lỗi usage cứng — không âm thầm
/// rơi về mặc định khiến CI ingest bất ngờ.
pub fn audit_unknown_format(value: &str) -> anyhow::Error {
    anyhow!("invalid audit --format '{value}' — expected table, json, sarif, or cyclonedx")
}

pub fn web_audit_needs_context() -> Error {
    anyhow!("web audit requires project adapter context")
}

// ===== run.rs =====

pub fn script_not_found(script: &str) -> Error {
    anyhow!(
        "Script '{script}' not found. Define it in 'mgc.toml' under [scripts] or in 'package.json'."
    )
}

pub fn forbidden_pm_script(cmd: &str, manifest: &std::path::Path, pm: &str) -> Error {
    anyhow!(
        "Script '{cmd}' in '{}' delegates to forbidden package manager '{pm}'. Core-web task execution must not bounce through another package manager.",
        manifest.display()
    )
}

pub fn unsupported_script(script: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!("Unsupported script '{script}': {e}")
}

/// Publish lifecycle hooks require explicit, one-command approval.
/// (Hook publish cần được cho phép tường minh cho lần chạy này.)
pub fn publish_lifecycle_opt_in_required() -> Error {
    anyhow!(
        "package defines publish lifecycle hooks; review them and rerun with --allow-scripts, or use --ignore-scripts to skip them"
    )
}

/// Malformed publish lifecycle metadata must never look like an empty hook list.
/// (Metadata lifecycle sai không được giả như danh sách hook rỗng.)
pub fn publish_lifecycle_policy_invalid() -> Error {
    anyhow!("package.json publish lifecycle scripts must be an object of string commands")
}

// ===== dlx.rs =====

pub fn dlx_no_binary(bin: &str, pkg: &str) -> Error {
    anyhow!("dlx: no binary '{bin}' found for package '{pkg}'. Is it an executable package?")
}

/// Reject malformed registry requests before creating dlx work directories.
/// Chặn request sai trước khi tạo project tạm, không lặp lại input nhạy cảm.
pub fn dlx_invalid_request(reason: &str) -> Error {
    anyhow!("dlx: invalid registry package request: {reason}")
}

/// Refuse unreadable caller policy without exposing configuration credential values.
/// Không in dòng TOML có thể chứa credential khi policy caller hỏng.
pub fn dlx_invalid_caller_policy() -> Error {
    anyhow!("dlx: cannot read caller project policy; fix mgc.toml before running dlx")
}

/// Refuse redirected cache directories and unexpected executable targets.
/// Không cho cache symlink hoặc executable thoát project tạm được chạy.
pub fn dlx_cache_path_unsafe(path: &std::path::Path) -> Error {
    anyhow!(
        "dlx: cache directory '{}' must be a real directory without symlinks",
        path.display()
    )
}

pub fn dlx_binary_outside_project() -> Error {
    anyhow!("dlx: executable resolves outside the verified temporary project")
}

// ===== hooks.rs =====

pub fn hook_failed(event: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!("hook {event} failed: {e}")
}

// ===== template.rs =====

pub fn template_download_url_invalid() -> Error {
    anyhow!(
        "template downloads require credential-free HTTPS URLs, or HTTP on a loopback registry without HTTPS downgrade"
    )
}

pub fn template_request_failed() -> Error {
    anyhow!("template registry request failed or exceeded its timeout")
}

pub fn template_metadata_invalid() -> Error {
    anyhow!("template registry returned invalid JSON metadata")
}

pub fn template_redirect_invalid() -> Error {
    anyhow!("template registry returned an invalid redirect or exceeded the redirect limit")
}

#[derive(Debug)]
pub struct TemplateHttpStatus(pub u16);

impl std::fmt::Display for TemplateHttpStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "template download failed with HTTP {}", self.0)
    }
}

impl std::error::Error for TemplateHttpStatus {}

#[derive(Debug)]
pub struct TemplateTransportUnavailable;

impl std::fmt::Display for TemplateTransportUnavailable {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("template registry transport is unavailable or timed out")
    }
}

impl std::error::Error for TemplateTransportUnavailable {}

pub fn template_transport_unavailable() -> Error {
    anyhow!(TemplateTransportUnavailable)
}

pub fn template_download_status(status: u16) -> Error {
    anyhow!(TemplateHttpStatus(status))
}

pub fn template_download_too_large(limit: usize) -> Error {
    anyhow!("template response exceeds the {limit} byte limit")
}

pub fn template_integrity_invalid() -> Error {
    anyhow!("template archive requires a valid SHA-256, SHA-384, or SHA-512 SRI digest")
}

pub fn template_integrity_mismatch() -> Error {
    anyhow!("template archive integrity mismatch")
}

pub fn template_layer_identity_invalid() -> Error {
    anyhow!(
        "template core/name must identify a supported core and a relative layer without traversal, drive prefixes, or control characters"
    )
}

pub fn template_cache_unsafe_path(path: &std::path::Path) -> Error {
    anyhow!(
        "refusing unsafe, linked, or non-directory template cache path '{}'",
        path.display()
    )
}

pub fn template_archive_layout_invalid() -> Error {
    anyhow!("template archive must contain one package directory with template.toml and sources")
}

pub fn template_cache_rollback_failed(
    target: &std::path::Path,
    retained: &std::path::Path,
    publish: &dyn std::fmt::Display,
    rollback: &dyn std::fmt::Display,
) -> Error {
    anyhow!(
        "cannot publish template cache '{}': {publish}; rollback failed: {rollback}; previous cache retained under '{}'",
        target.display(),
        retained.display()
    )
}

pub fn registry_missing_field(field: &str) -> Error {
    anyhow!("Registry response missing {field}")
}

// ===== registry/login =====

pub fn no_token_in_response() -> Error {
    anyhow!("registry response did not contain a usable token")
}

pub fn registry_auth_url_invalid() -> Error {
    anyhow!(
        "registry authentication requires a valid HTTP(S) URL without credentials, query, or fragment"
    )
}

pub fn registry_auth_requires_https() -> Error {
    anyhow!("registry authentication requires HTTPS except for loopback registry URLs")
}

pub fn registry_auth_request_failed() -> Error {
    anyhow!("registry authentication request failed or exceeded its timeout")
}

pub fn registry_auth_status_failed(status: u16) -> Error {
    anyhow!("registry authentication failed with HTTP {status}")
}

pub fn registry_auth_response_too_large() -> Error {
    anyhow!("registry authentication response exceeds the size limit")
}

pub fn registry_auth_response_invalid() -> Error {
    anyhow!("registry authentication response is not valid JSON")
}

pub fn registry_auth_password_required() -> Error {
    anyhow!("password input was empty")
}

pub fn registry_auth_username_required() -> Error {
    anyhow!("username is required")
}

pub fn registry_auth_password_too_large() -> Error {
    anyhow!("password input exceeds the size limit")
}

pub fn no_home_dir() -> Error {
    anyhow!("No home directory")
}

/// Refuse benchmark mode that would mutate the lockfile outside the install writer lock.
/// Từ chối benchmark có thể sửa lockfile ngoài writer lock của installer.
pub fn bench_no_lock_unsafe() -> Error {
    anyhow!(
        "bench --no-lock is disabled because deleting mgc.lock outside the install writer lock can race with project mutations"
    )
}

// ===== info.rs =====

pub fn info_no_web_adapter() -> Error {
    anyhow!(
        "package info currently requires the web core registry adapter, which is not included in this build"
    )
}

pub fn registry_fetch_failed(package: &str, e: &dyn std::fmt::Display) -> Error {
    let _ = package;
    anyhow!("Could not fetch package info from registry: {e}")
}

// ===== outdated.rs =====

pub fn outdated_no_web_adapter() -> Error {
    anyhow!(
        "outdated for the web core requires the web registry adapter, which is not included in this build"
    )
}

pub fn outdated_lock_unusable(core: &str) -> Error {
    anyhow!(
        "`mgc outdated` for core `{core}` requires a valid mgc.lock matching the current manifest; run `mgc install` to create or refresh it"
    )
}

// ===== web_registry_config.rs =====

pub fn registry_url_https() -> Error {
    anyhow!("registry URL must use HTTPS unless it targets localhost")
}

// ===== web.rs =====

pub fn topo_order_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!("monorepo topo order failed: {e}")
}

pub fn install_slot_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!("failed to acquire install slot: {e}")
}

pub fn install_task_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!("monorepo install task failed: {e}")
}

pub fn no_framework_specified() -> Error {
    anyhow!(
        "No framework specified. Use a flag like --react, --next, --vue, or pass a framework name as the first argument."
    )
}

pub fn pm_not_supported(pm: &str) -> Error {
    anyhow!(
        "--pm {pm} is not supported by core-web. MagiCore web uses the native mgc installer, not npm/pnpm/yarn/bun."
    )
}

pub fn ts_js_exclusive() -> Error {
    anyhow!("--ts and --js are mutually exclusive")
}

pub fn multiple_frameworks() -> Error {
    anyhow!(
        "Multiple frontend frameworks specified. Choose only one (--react, --next, --vue, etc.)."
    )
}

pub fn pinia_requires_vue() -> Error {
    anyhow!("--pinia requires --vue or --nuxt")
}

pub fn no_primary_package(fw: &str) -> Error {
    anyhow!("framework '{fw}' does not declare a primary package")
}

pub fn network_error_fetching(package: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!("network error fetching '{package}': {e}")
}

pub fn npm_registry_status(package: &str, status: &reqwest::StatusCode) -> Error {
    anyhow!("npm registry returned {status} for '{package}'")
}

pub fn bad_npm_response(package: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!("bad npm response for '{package}': {e}")
}

pub fn no_version_field(package: &str) -> Error {
    anyhow!("no version field for '{package}'")
}

/// Registry metadata cannot be used to enforce a scaffold age policy.
/// Không thể dùng metadata registry để áp dụng age policy khi scaffold.
pub fn web_scaffold_age_metadata_unavailable(
    package: &str,
    cause: &dyn std::fmt::Display,
) -> Error {
    anyhow!(
        "cannot enforce minimum-release-age for scaffold package '{package}': registry metadata is unavailable: {cause}"
    )
}

/// The project security configuration could not be loaded for web scaffolding.
/// Không đọc được security config của project khi scaffold web.
pub fn web_scaffold_security_config_unreadable(
    project_root: &Path,
    cause: &dyn std::fmt::Display,
) -> Error {
    anyhow!(
        "cannot load web scaffold security policy from '{}': {cause}",
        project_root.display()
    )
}

/// The project store could not provide its persisted web release policy.
/// Store của project không đọc được release policy web đã lưu.
pub fn web_scaffold_store_policy_unreadable(path: &Path, cause: &dyn std::fmt::Display) -> Error {
    anyhow!(
        "cannot read web scaffold release policy from '{}': {cause}",
        path.display()
    )
}

/// The project store policy database must be a regular file, not a link or special file.
/// Database policy của project phải là file thường, không phải link hay special file.
pub fn web_scaffold_store_policy_path_invalid(path: &Path) -> Error {
    anyhow!(
        "web scaffold release policy database '{}' must be a regular file",
        path.display()
    )
}

/// npm metadata omitted its latest tag for a scaffold package.
/// npm metadata thiếu latest tag cho package scaffold.
pub fn web_scaffold_latest_tag_missing(package: &str) -> Error {
    anyhow!("npm metadata for '{package}' has no latest dist-tag")
}

/// npm metadata's latest tag is not a valid semantic version.
/// latest tag trong npm metadata không phải semantic version hợp lệ.
pub fn web_scaffold_latest_tag_invalid(package: &str) -> Error {
    anyhow!("npm latest dist-tag for '{package}' is invalid")
}

/// npm metadata does not contain a user-selected dist-tag.
/// npm metadata không có dist-tag mà người dùng đã chọn.
pub fn web_scaffold_dist_tag_missing(package: &str, tag: &str) -> Error {
    anyhow!("npm metadata for '{package}' has no '{tag}' dist-tag")
}

/// The selected npm dist-tag does not point at a valid semantic version.
/// Dist-tag được chọn không trỏ tới semantic version hợp lệ.
pub fn web_scaffold_dist_tag_invalid(package: &str, tag: &str) -> Error {
    anyhow!("npm '{tag}' dist-tag for '{package}' is invalid")
}

/// The selected npm dist-tag points at a version blocked by the age policy.
/// Version của dist-tag bị policy tuổi phát hành chặn.
pub fn web_scaffold_dist_tag_not_eligible(
    package: &str,
    tag: &str,
    version: &mgc_types::Version,
) -> Error {
    anyhow!(
        "npm '{tag}' dist-tag for '{package}' points to {version}, which does not satisfy the configured minimum-release-age policy"
    )
}

/// No stable npm release satisfies the active minimum-release-age policy.
/// Không có bản stable nào thỏa policy tuổi phát hành tối thiểu.
pub fn web_scaffold_no_eligible_release(package: &str) -> Error {
    anyhow!("no stable release of '{package}' satisfies the configured minimum-release-age policy")
}

/// A curated scaffold range contains no stable version allowed by the age policy.
/// Range seed không chứa bản stable nào được age policy cho phép.
pub fn web_scaffold_no_eligible_release_in_range(package: &str, range: &str) -> Error {
    anyhow!(
        "no stable release of '{package}' in scaffold range '{range}' satisfies the configured minimum-release-age policy"
    )
}

pub fn package_json_root_object() -> Error {
    anyhow!("package.json root must be an object")
}

/// package.json scripts must remain a map of string commands.
/// Bảng scripts trong package.json phải giữ cấu trúc map lệnh dạng chuỗi.
pub fn package_json_scripts_object() -> Error {
    anyhow!("package.json scripts must be an object of string commands")
}

/// dev chưa implement cho core này
pub fn dev_core_not_implemented(core: &str) -> Error {
    anyhow!("'mgc dev' Engine is not implemented for the '{core}' core yet")
}

/// dev chưa implement cho iot framework này
pub fn dev_iot_framework_not_implemented(fw: &str) -> Error {
    anyhow!("'mgc dev' for '{fw}' iot framework is not implemented yet")
}

/// dev chưa implement cho cloud type này
pub fn dev_cloud_not_implemented(cloud: &str) -> Error {
    anyhow!(
        "'mgc dev' for '{cloud}' has no MagiCore-native provider engine yet; no external provider CLI was started"
    )
}

// ===== web.rs (dev/install/script) =====

pub fn web_no_dev_target(root: &std::path::Path, hint: &str) -> Error {
    anyhow!(
        "No runnable dev target found in '{}'. Run '{}' first.",
        root.display(),
        hint
    )
}

pub fn web_workspace_dep_missing(
    dep: &str,
    target: &std::path::Path,
    root: &std::path::Path,
) -> Error {
    anyhow!(
        "workspace dependency '{dep}' referenced by '{}' was not found under '{}'",
        target.display(),
        root.display()
    )
}

pub fn web_empty_dev_script(root: &std::path::Path) -> Error {
    anyhow!(
        "Empty dev script in '{}'",
        root.join("package.json").display()
    )
}

pub fn web_unsupported_dev_script(script: &str, root: &std::path::Path) -> Error {
    anyhow!(
        "Unsupported dev script '{script}' in '{}'",
        root.join("package.json").display()
    )
}

pub fn web_forbidden_pm(script: &str, manifest: &std::path::Path, pm: &str) -> Error {
    anyhow!(
        "Unsupported script '{script}' in '{}': it delegates to '{pm}'. Core-web must execute natively through MagiCore or framework-local binaries, not through another package manager.",
        manifest.display()
    )
}

pub fn web_no_dev_entrypoint(root: &std::path::Path) -> Error {
    anyhow!(
        "No 'dev' script found in '{}' and no supported native dev entrypoint was detected",
        root.display()
    )
}

/// A dev process group received SIGINT/SIGTERM.
/// Nhóm dev nhận SIGINT/SIGTERM và đang được dọn dẹp.
pub fn web_dev_interrupted() -> Error {
    anyhow!("dev server group interrupted by SIGINT/SIGTERM")
}

/// A sibling dev target ended and this target is being cleaned up.
/// Target dev cùng nhóm kết thúc nên target này đang được dọn dẹp.
pub fn web_dev_target_cancelled() -> Error {
    anyhow!("dev target cancelled because a sibling target stopped")
}

pub fn web_no_install_flow(root: &std::path::Path) -> Error {
    anyhow!(
        "No supported native install flow found in '{}'",
        root.display()
    )
}

pub fn web_missing_executable(bin: &str, hint: &str, root: &std::path::Path) -> Error {
    anyhow!(
        "Missing local executable '{bin}'. Run '{hint}' in '{}'.",
        root.display()
    )
}

// ===== build.rs / install.rs =====

pub fn build_toolchain_missing(tool: &str) -> Error {
    anyhow!(
        "'{tool}' not found in PATH — install it first (mgc doctor check lists required toolchains)"
    )
}

pub fn java_build_backend_unavailable(root: &std::path::Path) -> Error {
    anyhow!(
        "Java build descriptor found in '{}', but MagiCore has no native Java build backend yet; refusing to route the project through TypeScript or invoke Maven/Gradle",
        root.display()
    )
}

/// Build has no supported artifact for the selected core — core này chưa có artifact build được hỗ trợ.
pub fn build_not_supported(core: &str, guidance: &str) -> Error {
    anyhow!("`mgc build` is not supported for core '{core}': {guidance}")
}

/// Build completed without producing any artifact — build không tạo được artifact nào.
pub fn build_no_artifact() -> Error {
    anyhow!(
        "build produced no artifact; fix the project configuration or install the required toolchain"
    )
}

/// Refuse to infer an App build target from a malformed `mgc.toml`.
pub fn app_build_config_invalid(path: &std::path::Path, error: &dyn std::fmt::Display) -> Error {
    anyhow!("invalid App build config '{}': {error}", path.display())
}

/// Report App config I/O errors instead of treating unreadable config as absent.
pub fn app_build_config_read_failed(
    path: &std::path::Path,
    error: &dyn std::fmt::Display,
) -> Error {
    anyhow!("cannot read App build config '{}': {error}", path.display())
}

/// Explicit App language must have a build implementation.
pub fn app_build_language_invalid(language: &str) -> Error {
    anyhow!("unsupported or invalid App build language '{language}'")
}

/// Multiple App build runtimes require explicit project configuration.
/// Nhiều runtime App cần được khai báo tường minh trong cấu hình dự án.
pub fn app_build_language_ambiguous(languages: &[&str]) -> Error {
    anyhow!(
        "App build runtime is ambiguous ({}); set `app.language` explicitly",
        languages.join(", ")
    )
}

/// App multi-platform configuration must be explicit and well-formed.
/// Cấu hình đa nền tảng của App phải tường minh và hợp lệ.
pub fn app_build_platforms_invalid(reason: &str) -> Error {
    anyhow!("invalid App build platforms configuration: {reason}")
}

/// A multi-platform App build is incomplete when any selected target was skipped.
pub fn build_multi_platforms_incomplete(skipped: &[String]) -> Error {
    anyhow!(
        "multi-platform build incomplete; selected target(s) not built: {}",
        skipped.join(", ")
    )
}

pub fn workspace_failed(count: usize) -> Error {
    anyhow!("{count} workspace(s) failed to install")
}

// ===== context.rs =====

pub fn unknown_ecosystem(ecosystem: &str) -> Error {
    anyhow!("Unknown ecosystem: '{ecosystem}'")
}

pub fn no_registry_configured() -> Error {
    anyhow!("no registry configured")
}

pub fn cannot_detect_project_type(root: &std::path::Path) -> Error {
    anyhow!(
        "Cannot detect project type in '{}'. Run `mgc init --template <type>`, \
         `mgc init --signature <core>` to write a plain-text core marker, or specify `--core <type>`",
        root.display()
    )
}

pub fn no_mgc_project_root() -> Error {
    anyhow!("No MagiCore project found. Run `mgc init` to create one, or specify `--core <type>`.")
}

// ===== factory.rs =====

pub fn core_not_in_build(core: &str) -> Error {
    anyhow!("'{core}' core is not included in this build.")
}

pub fn detect_core_failed(kind: &str) -> Error {
    let msg = match kind {
        "game" => {
            "Cannot detect a game project here (missing mgc.toml/project.godot/manifest.json/.uproject)."
        }
        "ai" => {
            "Cannot detect an ai project here (missing mgc.toml [ai] framework / pyproject [tool.magicore] framework)."
        }
        "clo" | "cloud" => {
            "Cannot detect a cloud project here (missing mgc.toml/Pulumi.yaml/*.tf/cdk package.json)."
        }
        "cicd" => {
            "Cannot detect a cicd project here (missing mgc.toml/wrangler.toml/argocd/.github/workflows)."
        }
        "iot" => "Cannot detect an iot project here (missing mgc.toml/platformio.ini/west.yml).",
        "app" => {
            "Cannot detect an app project here (missing mgc.toml/pubspec.yaml/build.gradle/Package.swift)."
        }
        "hardware" => {
            "Cannot detect a hardware project here (missing mgc.toml with ecosystem = \"hardware\")."
        }
        "lib" => "Cannot detect a lib project here (missing mgc.toml/lib marker).",
        _ => "Cannot detect a project here.",
    };
    anyhow!(msg)
}

// ===== bundler/mod.rs =====

pub fn esbuild_failed(msgs: &[String]) -> Error {
    anyhow!("esbuild failed: {}", msgs.join("\n"))
}

pub fn link_node_modules_failed(
    source: &std::path::Path,
    target: &std::path::Path,
    err: &dyn std::fmt::Display,
) -> Error {
    anyhow!(
        "failed to link node_modules for build from {} to {}: {}",
        source.display(),
        target.display(),
        err
    )
}

// ===== scaffold/processor.rs =====

pub fn unsupported_web_framework(framework: &str) -> Error {
    anyhow!("Unsupported web framework '{framework}'")
}

pub fn dir_already_exists(target: &std::path::Path) -> Error {
    anyhow!("Directory '{}' already exists", target.display())
}

pub fn scaffold_staging_failed(target: &std::path::Path, cause: Error) -> Error {
    anyhow!(
        "Scaffolding '{}' failed atomically; no partial project was left \
         behind. Cause: {cause}",
        target.display()
    )
}

pub fn create_claim_conflict(claim: &std::path::Path) -> Error {
    anyhow!(
        "Another create for this project name appears to be in progress \
         (claim file '{}' exists). If no create is running, remove the claim \
         file and retry.",
        claim.display()
    )
}

pub fn invalid_project_name_retries_exhausted(retries: u32) -> Error {
    anyhow!(
        "Project name rejected after {retries} attempts; wizard aborted \
         without creating anything (no fallback name is invented)"
    )
}

pub fn invalid_project_name(project_name: &str) -> Error {
    anyhow!(
        "Invalid project name '{project_name}': must be a single path segment without \
         separators, '..', or absolute paths (path traversal rejected)"
    )
}

pub fn unsupported_scaffold_core(core: &str) -> Error {
    anyhow!("Unsupported core '{core}'")
}

pub fn unsupported_scaffold_framework(framework: &str) -> Error {
    anyhow!("Unsupported scaffold framework '{framework}'")
}

pub fn web_template_path_missing(dir: &str) -> Error {
    anyhow!("Web template path '{dir}' does not exist")
}

pub fn unsupported_web_mode(mode: &str) -> Error {
    anyhow!("Unsupported web mode '{mode}'")
}

pub fn web_template_layer_missing(layer: &str) -> Error {
    anyhow!("Web template layer '{layer}' does not exist")
}

pub fn unsupported_fullstack_framework(combined: &str) -> Error {
    anyhow!("Unsupported fullstack framework '{combined}' for composited web scaffold")
}

pub fn web_scaffold_needs_fe_be() -> Error {
    anyhow!("Web scaffold needs a frontend and backend framework")
}

pub fn unsupported_monorepo_backend(backend: &str) -> Error {
    anyhow!("Unsupported monorepo backend '{backend}'")
}

pub fn unsupported_monorepo_frontend(frontend: &str) -> Error {
    anyhow!("Scaffold for monorepo frontend framework '{frontend}' is not implemented yet")
}

pub fn scaffold_not_implemented(label: &str, layer: &str) -> Error {
    anyhow!(
        "Scaffold for {} is not implemented yet at '{}'",
        label,
        layer
    )
}

pub fn template_layer_missing_manifest(layer: &str) -> Error {
    anyhow!("Template layer '{layer}' missing template.toml")
}

pub fn duplicate_template_target(target: &str, layer: &str) -> Error {
    anyhow!("Duplicate template target '{target}' in '{layer}'")
}

pub fn template_source_missing(source: &str, layer: &str) -> Error {
    anyhow!("Template source '{source}' does not exist in '{layer}'")
}

pub fn binary_source_with_context(label: &str) -> Error {
    anyhow!("Binary template source '{label}' cannot declare template context")
}

pub fn template_token_undeclared(token: &str, source: &str) -> Error {
    anyhow!("Template token '{token}' in '{source}' is not declared in template.toml")
}

pub fn template_token_unsupported(token: &str, source: &str) -> Error {
    anyhow!("Template token '{token}' in '{source}' is not supported by the Rust compiler context")
}

pub fn template_context_unsupported(key: &str, source: &str) -> Error {
    anyhow!(
        "Template context '{key}' required by '{source}' is not supported by the Rust compiler context"
    )
}

pub fn template_manifest_missing(layer: &str) -> Error {
    anyhow!("Missing template manifest '{layer}'")
}

// ===== config cmd =====

pub fn config_key_missing(key: &str) -> Error {
    anyhow!("config key '{key}' not found (checked env, project .npmrc, and ~/.npmrc)")
}

pub fn config_core_identity_managed_by_signature() -> Error {
    anyhow!(
        "the project core identity is immutable through `mgc config`; `mgc init --signature <core>` writes a plain-text marker and cannot reassign an existing project"
    )
}

/// Refuse a per-core command when another core owns the project directory.
/// Từ chối lệnh core nếu project do core khác sở hữu.
pub fn project_core_identity_mismatch(
    project_root: &std::path::Path,
    expected: &str,
    detected: &str,
) -> Error {
    anyhow!(
        "project '{}' belongs to core '{}'; refusing to run the '{}' core command",
        project_root.display(),
        detected,
        expected,
    )
}

/// Refuse to route a per-core command when project identity cannot be proven.
/// Không định tuyến lệnh core khi chưa xác minh được identity project.
pub fn project_core_identity_missing(project_root: &std::path::Path, expected: &str) -> Error {
    anyhow!(
        "cannot prove project core identity for '{}'; refusing to run the '{}' core command",
        project_root.display(),
        expected,
    )
}

pub fn core_selector_conflicts_command(command_core: &str, selected_core: &str) -> Error {
    anyhow!(
        "global --core '{}' conflicts with the explicit '{}' core command",
        selected_core,
        command_core,
    )
}

pub fn config_project_file_symlink(path: &std::path::Path) -> Error {
    anyhow!("project config '{}' must not be a symlink", path.display())
}

pub fn config_project_file_not_regular(path: &std::path::Path) -> Error {
    anyhow!("project config '{}' must be a regular file", path.display())
}

pub fn config_project_file_invalid_path() -> Error {
    anyhow!("configuration path has no file name")
}

// ===== global flags =====

pub fn dir_missing(dir: &str, cause: String) -> Error {
    anyhow!("cannot change to directory '{dir}': {cause}")
}

pub fn runtime_dangerous_flag_rejected(runtime: &str, flag: &str) -> Error {
    anyhow!("Rejected dangerous {runtime} flag: {flag}. Use project scripts only.")
}

pub fn runtime_dangerous_permission_rejected(runtime: &str, permission: &str) -> Error {
    anyhow!(
        "Rejected dangerous {runtime} permission: {permission}. Explicitly allow in deno.json tasks if needed."
    )
}

// ===== dependency ownership gate (C0 firewall, T0.3) =====

/// Legacy delegated operation is blocked because dependency compatibility is disabled.
pub fn dep_gate_requires_compat(
    core: &str,
    op: &str,
    ecosystem: Option<&str>,
    tools: &[&str],
) -> Error {
    let lane = match ecosystem {
        Some(eco) => format!("`{core}` {op} (ecosystem '{eco}')"),
        None => format!("`{core}` {op}"),
    };
    anyhow!(
        "{lane} is unsupported by MagiCore's native dependency engine (known external tools: {}) — external package-manager execution is disabled for dependency operations",
        tools.join(", ")
    )
}

/// An operation with no dependency lifecycle at all (no engine, no lane).
/// cicd/hardware lanes are scaffold-only by design — the message says so
/// instead of letting anyone read them as package installs.
pub fn dep_gate_unsupported(core: &str, op: &str, ecosystem: Option<&str>) -> Error {
    let lane = match ecosystem {
        Some(eco) => format!("`{core}` {op} (ecosystem '{eco}')"),
        None => format!("`{core}` {op}"),
    };
    if matches!(core, "cicd" | "hardware") {
        anyhow!(
            "{lane} has no package lifecycle — this is a scaffold-only lane (templates/pipelines/generators), never a native package install"
        )
    } else {
        anyhow!(
            "{lane} is unsupported for dependency lifecycle — the manifest is toolchain-owned, but no approved compatibility runner is configured for this operation"
        )
    }
}

/// Refuse dependency operations until MagiCore owns their complete native
/// lifecycle; this deliberately offers no toolchain/compatibility escape.
/// Từ chối lifecycle dependency khi MagiCore chưa sở hữu engine native;
/// không mở đường vòng qua package manager ngoài.
pub fn native_dependency_engine_unavailable(core: &str, ecosystem: &str, op: &str) -> Error {
    anyhow!(
        "`{core}` {op} for `{ecosystem}` is unavailable: MagiCore does not yet own the complete native dependency lifecycle for this lane. No external package manager was invoked; use a supported native lane or wait for native resolver, lock, fetch, verify, store, and materializer support."
    )
}

/// Store commands currently manipulate only the Web-owned store layout.
pub fn store_requires_web_core(core: Option<&str>) -> Error {
    match core {
        Some(core) => anyhow!(
            "MagiCore store commands currently support only Web projects; this project is marked as core '{core}'"
        ),
        None => anyhow!(
            "MagiCore store commands require a valid .mgc.core core marker or mgc.toml core; the store is Web-only"
        ),
    }
}

/// Store commands require an explicit project core marker.
pub fn store_core_signature_required() -> Error {
    anyhow!(
        "MagiCore store commands require a valid .mgc.core core marker; this plain-text marker is not a cryptographic signature, so confirm this is a Web project before running `mgc init --signature web`"
    )
}

/// Refuse store access when the project signature and project config disagree.
pub fn store_core_identity_conflict(marker: &str, configured: &str) -> Error {
    anyhow!(
        "project core identity conflict: .mgc.core says '{marker}' but mgc.toml says '{configured}'; refusing Web store access"
    )
}

/// Store doctor requires a core marker matching the requested native lane.
/// Store doctor cần marker core khớp lane native được yêu cầu.
pub fn store_core_marker_required(core: &str) -> Error {
    anyhow!("store doctor requires a valid .mgc.core marker for core '{core}'")
}

/// Refuse shared-store access when project markers disagree.
/// Từ chối shared-store nếu marker project không thống nhất.
pub fn store_core_identity_conflict_any(marker: &str, configured: &str) -> Error {
    anyhow!(
        "project core identity conflict: .mgc.core says '{marker}' but mgc.toml says '{configured}'; refusing store access"
    )
}

/// Refuse shared-store access when markers identify another core.
/// Từ chối shared-store nếu marker chỉ sang core khác.
pub fn store_core_identity_mismatch(expected: &str, marker: &str, configured: &str) -> Error {
    anyhow!(
        "store doctor requested core '{expected}' but project markers identify '{marker}' and '{configured}'"
    )
}

/// Store doctor requires framework metadata to resolve a native store route.
/// Store doctor cần framework metadata để tìm native store route.
pub fn store_doctor_framework_metadata_required(core: &str, framework: &str) -> Error {
    anyhow!(
        "store doctor cannot verify core '{core}' framework '{framework}' because mgc.toml is missing"
    )
}

/// Refuse to inspect a route that disagrees with the project's framework declaration.
/// Từ chối kiểm tra route không khớp khai báo framework của project.
pub fn store_doctor_framework_mismatch(core: &str, requested: &str, declared: &[String]) -> Error {
    let declared = if declared.is_empty() {
        "<none>".to_string()
    } else {
        declared.join(", ")
    };
    anyhow!(
        "store doctor requested core '{core}' framework '{requested}' but mgc.toml declares '{declared}'"
    )
}

/// Unsupported store/framework pair; never inspect a different store by guess.
/// Cặp core/framework chưa hỗ trợ; không đoán sang store khác.
pub fn store_doctor_route_unsupported(core: &str, framework: Option<&str>) -> Error {
    match framework {
        Some(framework) => anyhow!(
            "store doctor has no MagiCore-owned store route for core '{core}' framework '{framework}'"
        ),
        None => anyhow!(
            "store doctor requires --framework for core '{core}' because its store route is framework-specific"
        ),
    }
}

/// Global shared-root doctor has no safe automatic repair operation.
/// Doctor shared root global chưa có thao tác repair an toàn tự động.
pub fn store_doctor_repair_unsupported(core: &str) -> Error {
    anyhow!(
        "store doctor --repair is unsupported for core '{core}' shared roots; no files were changed"
    )
}

/// One or more configured shared roots failed the path-safety check.
/// Một hoặc nhiều shared root không qua kiểm tra an toàn đường dẫn.
pub fn store_global_root_unsafe() -> Error {
    anyhow!("store doctor: one or more MGC-managed store roots are unsafe")
}

/// Give a recovery command that retains the core/framework scope being checked.
/// Gợi ý lệnh chạy lại mà giữ nguyên core/framework đang kiểm tra.
pub fn store_doctor_unhealthy(core: &str, framework: Option<&str>) -> Error {
    match framework {
        Some(framework) => anyhow!(
            "store doctor: NEEDS REPAIR — prune/install refuse until fixed (re-run `mgc store doctor --core {core} --framework {framework}` for details)"
        ),
        None => anyhow!(
            "store doctor: NEEDS REPAIR — prune/install refuse until fixed (re-run `mgc store doctor --core {core}` for details)"
        ),
    }
}

/// Legacy compat flag named a tool for a lane without native ownership.
/// Compatibility execution is disabled; tool identity does not grant an exception.
pub fn dep_gate_wrong_tool(
    core: &str,
    op: &str,
    ecosystem: Option<&str>,
    got: &str,
    tools: &[&str],
) -> Error {
    let lane = match ecosystem {
        Some(eco) => format!("`{core}` {op} (ecosystem '{eco}')"),
        None => format!("`{core}` {op}"),
    };
    anyhow!(
        "`--compat-runtime '{got}'` cannot authorize {lane} (known external tools: {}) — external package-manager execution is disabled for dependency operations",
        tools.join(", ")
    )
}

/// The lane's exact tool is not in the owner set at all — a lane/table
/// mismatch (lane bug), failed closed, never silently run.
pub fn dep_gate_lane_tool_not_owned(
    core: &str,
    op: &str,
    ecosystem: Option<&str>,
    actual: &str,
    tools: &[&str],
) -> Error {
    let lane = match ecosystem {
        Some(eco) => format!("`{core}` {op} (ecosystem '{eco}')"),
        None => format!("`{core}` {op}"),
    };
    anyhow!(
        "lane tool '{actual}' does not own {lane} (owner toolchain: {}) — refusing to run (lane/table mismatch, fail-closed)",
        tools.join(", ")
    )
}

/// The opt-in names a DIFFERENT tool than the lane will actually spawn
/// (e.g. `--compat-runtime uv` on a lane that spawns pip): the flag must
/// equal the process (flag==process contract) — failed closed with both
/// names so the user sees the mismatch.
pub fn dep_gate_tool_mismatch(
    core: &str,
    op: &str,
    ecosystem: Option<&str>,
    wanted: &str,
    actual: &str,
    tools: &[&str],
) -> Error {
    let lane = match ecosystem {
        Some(eco) => format!("`{core}` {op} (ecosystem '{eco}')"),
        None => format!("`{core}` {op}"),
    };
    anyhow!(
        "`--compat-runtime '{wanted}'` does not own {lane}: this lane spawns '{actual}' (owner toolchain: {}) — pass `--compat-runtime {actual}` (the flag must name the process that will run)",
        tools.join(", ")
    )
}

/// Multi-platform install finished with skipped platforms (skip is not success).
pub fn app_multi_platforms_skipped(skipped: &[String]) -> Error {
    anyhow!(
        "install incomplete — skipped platform(s): {} (missing tool or runner; install them and re-run — a skip is never success)",
        skipped.join(", ")
    )
}

/// `install-lib` got package args: install replays the existing
/// graph/lock, it never adds (adding spawns the provider toolchain and
/// belongs to `add-lib` behind its own gate).
pub fn install_lib_packages_use_add(packages: &[String]) -> Error {
    anyhow!(
        "`install-lib` takes no packages (got {:?}) — install replays the existing graph/lock through the native pipeline; add packages with `mgc add-lib <package>`",
        packages
    )
}

/// `install-app` takes no packages — same split as install-lib.
pub fn install_app_packages_use_add(packages: &[String]) -> Error {
    anyhow!(
        "`install-app` takes no packages (got {:?}) — install replays the existing graph/lock through the native pipeline; add packages with `mgc add-app <package>`",
        packages
    )
}

/// `update` of a package the manifest does not declare — fail loudly
/// instead of adding it silently (add and update stay separate verbs).
pub fn update_unknown_package(name: &str) -> Error {
    anyhow!("cannot update '{name}': not in the project manifest — add it first with `mgc add`")
}

/// `mgc migrate` does not know this target (only `--to v4` exists).
pub fn migrate_unknown_target(to: &str) -> Error {
    anyhow!("unknown migrate target '{to}' (only `--to v4` exists)")
}

/// `mgc outdated` checked nothing: every registry fetch failed, so any
/// "up to date" verdict would be fabricated.
/// (outdated không check được gì — fail cứng thay vì báo láo.)
pub fn outdated_no_registry_response(failed: String) -> Error {
    anyhow!(
        "could not reach the registry for any dependency ({failed}) — refusing to report 'up to date' on zero evidence"
    )
}

/// `mgc migrate lock` found no mgc.lock to migrate.
pub fn migrate_no_lockfile(root: &std::path::Path) -> Error {
    anyhow!("no mgc.lock in '{}' — nothing to migrate", root.display())
}

/// `mgc migrate lock` refused: the v1/v2/v3 → v4 migration would drop
/// data (edges, peers, sources, or installer SRI). The existing mgc.lock
/// was left unchanged — re-resolve the lossy lanes first, then migrate.
/// (Migrate từ chối vì hao hụt dữ liệu — giữ nguyên lock cũ.)
pub fn migrate_v4_lossy(warnings: &[String]) -> Error {
    anyhow!(
        "schema v4 migration refused (lossy — existing mgc.lock left unchanged): {}",
        warnings.join("; ")
    )
}

/// `mgc migrate lock` emitted v4 text that does not round-trip into the
/// identical document — a serializer drift that must never hit disk.
/// (Text v4 ghi ra không round-trip y hệt — không được ghi đĩa.)
pub fn migrate_v4_roundtrip_mismatch() -> Error {
    anyhow!(
        "migrated v4 document does not round-trip identically — refusing to write (serializer drift)"
    )
}

/// `mgc migrate lock` cannot start from this schema version.
pub fn migrate_unsupported_version(version: u8) -> Error {
    anyhow!("cannot migrate lockfile schema version {version} (supported sources: v1, v2, v3)")
}

/// --compat-runtime value outside the known tool universe for dependency ops.
pub fn dep_gate_invalid_tool(tool: &str, valid: &[&str]) -> Error {
    anyhow!(
        "invalid --compat-runtime '{tool}' for dependency operations — accepted tool names: {}; compatibility execution is disabled for dependency operations",
        valid.join(", ")
    )
}

/// Dependency commands do not permit compatibility package-manager execution.
/// Dependency-op không cho phép chạy package manager qua compatibility.
pub fn dep_gate_compat_disabled(value: &str) -> Error {
    anyhow!(
        "--compat-runtime '{value}' is disabled for dependency operations: MagiCore will not invoke an external package manager; use a fully supported native lane"
    )
}

/// --version pinning is not implemented for this lane (its tool call has
/// no verified version semantics) — failed loudly instead of silently
/// dropping the pin.
/// (--version chưa hỗ trợ cho lane này — fail rõ thay vì nuốt version.)
pub fn add_version_unsupported(core: &str, pinned: &str) -> Error {
    anyhow!(
        "`{core}` add does not support `--version {pinned}` yet (no verified version semantics on this lane) — pin the version inside the package spec or omit --version"
    )
}

/// A package already carrying its own version spec combined with
/// `--version` is ambiguous — failed loudly, never merged silently.
pub fn add_version_conflict(package: &str, pinned: &str) -> Error {
    anyhow!(
        "package '{package}' already carries a version spec and `--version {pinned}` was also given — use one or the other"
    )
}

/// A web-pipeline-only flag was passed for a non-JS backend lane — failed loudly
/// instead of silently dropping the flag.
/// (Flag chỉ-dành-web pipeline dùng cho backend non-JS — fail rõ.)
pub fn web_backend_flag_unsupported(flag: &str, language: &str) -> Error {
    anyhow!(
        "`{flag}` applies to the JavaScript pipeline only and is not supported for the {language} backend lane — omit the flag"
    )
}

/// A native build output path contains a symlink or a non-directory component.
/// Đường dẫn output build native chứa symlink hoặc thành phần không phải thư mục.
pub fn build_output_path_unsafe(path: &std::path::Path, reason: &str) -> Error {
    anyhow!(
        "refusing native build output '{}': {reason}",
        path.display()
    )
}

/// Go build cannot prove dependency ownership from the current manifest shape.
/// Go build không chứng minh được dependency ownership từ dạng manifest hiện tại.
pub fn go_build_freshness_unverified(reason: &str) -> Error {
    anyhow!(
        "cannot verify Go dependency ownership before build: {reason}; remove unsupported go.mod replace directives or use a qualified native install flow"
    )
}

/// Generic install with package arguments on a manifest MagiCore cannot mutate.
/// Install tổng quát không được sửa manifest mà MagiCore chưa sở hữu.
pub fn install_packages_toolchain_owned(adapter_name: &str) -> Error {
    anyhow!(
        "generic install cannot add packages to the '{adapter_name}' manifest because MagiCore does not own its native dependency lifecycle; no external package manager was invoked"
    )
}

/// A rival JS runtime cannot be used as a hidden or explicit MGC engine.
/// Runtime JS đối thủ không được dùng làm engine ẩn hay opt-in của MGC.
pub fn rival_runtime_not_native(runtime: &str) -> Error {
    anyhow!(
        "'{runtime}' cannot run through MagiCore: the native MagiCore runtime for this project is not implemented; no external runtime was invoked"
    )
}
/// Monorepo member has no package.json and its toolchain was NOT opted
/// into: pass exactly `--compat-runtime=<tool>` for the member kind
/// (go/pip/cargo/mvn/composer) — one flag never opens every toolchain.
/// NOTE: takes the pre-formatted opt-in description (not CompatMode) so
/// this module stays includable from integration tests without the full
/// command tree.
/// (Thành viên monorepo không có package.json — phải opt-in ĐÚNG tool;
/// một flag không mở mọi toolchain.)
pub fn monorepo_compat_tool_denied(
    project_root: &std::path::Path,
    tool: Option<&str>,
    have_opt_in: &str,
) -> Error {
    match tool {
        Some(tool) => anyhow!(
            "monorepo member '{}' needs its '{tool}' toolchain, but the invocation opted into '{have_opt_in}' — pass --compat-runtime={tool} (explicit only, never auto)",
            project_root.display(),
        ),
        None => anyhow!(
            "monorepo member '{}' has no package.json and no recognized toolchain manifest (go.mod/requirements.txt/Cargo.toml/pom.xml/composer.json) — nothing to delegate to",
            project_root.display()
        ),
    }
}

/// A mutation adapter returned success but rereading the manifest shows no change.
/// Adapter báo thành công nhưng đọc lại manifest không thấy thay đổi.
pub fn tool_manifest_mismatch(package: &str, adapter: &str, detail: &str) -> Error {
    anyhow!(
        "'{package}' was not {detail} by {adapter}; MagiCore re-read the manifest and found no requested change"
    )
}

/// self-update: explicit version failed validation (anchored semver).
/// (Version self-update không hợp lệ.)
pub fn self_update_invalid_version(version: &str) -> Error {
    anyhow!("invalid version: {version} (expected like 1.1.0-rc.10, with or without a leading 'v')")
}

/// self-update: unknown release variant.
/// (Variant release không biết.)
pub fn self_update_unsupported_variant(variant: &str) -> Error {
    anyhow!("unsupported variant: {variant} (magicore|magicore-web)")
}

/// self-update: host platform outside the release matrix.
/// (Nền tảng máy không trong ma trận release.)
pub fn self_update_unsupported_host(detail: &str) -> Error {
    anyhow!("unsupported {detail} for self-update (linux/macos/windows on x64/arm64 only)")
}

/// self-update: checksum mismatch — abort before extract/swap.
/// (Checksum lệch — dừng trước mọi bước sau.)
pub fn self_update_checksum_mismatch(archive: &str, expected: &str, actual: &str) -> Error {
    anyhow!("checksum mismatch for {archive}: expected {expected}, got {actual}")
}

/// self-update: archive contains no mgc binary.
/// (Archive không chứa binary mgc.)
pub fn self_update_no_binary(archive: &str) -> Error {
    anyhow!("archive {archive} contains no mgc binary")
}

/// self-update: download failed or over quota.
/// (Tải thất bại hoặc quá quota.)
pub fn self_update_download_failed(url: &str, detail: &str) -> Error {
    anyhow!("download {url} failed: {detail}")
}

/// self-update: new binary failed its launch probe or reported the
/// wrong version — previous version restored.
/// (Binary mới fail probe — đã rollback.)
pub fn self_update_probe_failed(detail: &str) -> Error {
    anyhow!("new binary failed verification ({detail}) — previous version restored")
}

/// self-update: release manifest is missing, malformed, or does not
/// bind this exact artifact.
/// (Manifest release thiếu/sai/không khớp artifact.)
pub fn self_update_manifest_invalid(detail: &str) -> Error {
    anyhow!("release manifest rejected: {detail}")
}

/// self-update: signature verification failed against configured trust roots.
/// (Chữ ký manifest không verify được.)
pub fn self_update_signature_invalid(detail: &str) -> Error {
    anyhow!("release manifest signature invalid: {detail}")
}

/// self-update: unsigned fallback refused — fail-closed provenance.
/// (Từ chối đường unsigned — provenance fail-closed.)
pub fn self_update_unsigned_refused(detail: &str) -> Error {
    anyhow!("unsigned self-update refused: {detail}")
}

/// `mgc dev` pre-flight: manifest declares dependencies missing from the lock.
/// (Pre-flight `mgc dev`: manifest khai báo dependency không có trong lock.)
pub fn stale_lock_needs_install(detail: &str) -> Error {
    anyhow!(
        "project dependencies changed since mgc.lock was written ({detail}) — run `mgc install` first"
    )
}

/// A dependency manifest has an invalid shape for freshness projection.
/// Manifest dependency có cấu trúc không hợp lệ để kiểm freshness.
pub fn freshness_manifest_invalid(file: &str, detail: &str) -> Error {
    anyhow!("cannot check dependency freshness: {file} {detail}")
}

/// A signed policy cache update must not roll back issue time or revocations.
/// Từ chối cập nhật cache policy có ký nếu lùi thời gian hoặc mất revocation.
pub fn policy_manifest_update_rejected(detail: &str) -> Error {
    anyhow!("signed policy manifest update rejected: {detail}")
}

/// A policy cache could not acquire its cross-process writer lock.
/// Không lấy được khóa ghi xuyên tiến trình cho cache policy.
pub fn policy_manifest_cache_lock_failed(detail: &str) -> Error {
    anyhow!("cannot lock signed policy manifest cache: {detail}")
}

/// `--compat-runner` on a core without a qualified compat contract.
/// (`--compat-runner` trên core chưa có compat contract.)
pub fn compat_runner_unsupported_core(core: &str) -> Error {
    anyhow!(
        "--compat-runner is only supported for the web core right now; refusing to claim compat parity for '{core}'"
    )
}

/// Pre-flight profile gate: this lane observably requires package managers
/// at runtime, so native mode refuses before spawning anything.
/// (Cổng profile pre-flight: lane này chắc chắn cần package manager lúc
/// runtime, nên native mode từ chối trước khi spawn.)
pub fn lane_needs_compat_runner(framework: &str, managers: &[String]) -> Error {
    anyhow!(
        "lane '{framework}' requires package managers at runtime ({}) — native mode refuses before spawning; rerun with --compat-runner <manager> or use a native alternative",
        managers.join(", ")
    )
}

/// Refuse native dev when a stored spawn profile exists but cannot be read.
/// (Từ chối native dev khi spawn profile đã lưu tồn tại nhưng không đọc được.)
pub fn invalid_spawn_profile(detail: &str) -> Error {
    anyhow!("invalid spawn profile; refusing native dev pre-flight: {detail}")
}

/// Reject executable selection that cannot be attributed to the requested package.
/// Từ chối bin sai tên, mơ hồ hoặc target không thuộc package được yêu cầu.
pub fn dlx_invalid_bin() -> Error {
    anyhow!("dlx: invalid executable name; use a simple declared bin name")
}
pub fn dlx_undeclared_bin() -> Error {
    anyhow!("dlx: selected executable is not declared by the requested package")
}
pub fn dlx_ambiguous_bin() -> Error {
    anyhow!("dlx: package declares multiple executables; select one with --bin NAME")
}
pub fn dlx_invalid_package_bins() -> Error {
    anyhow!("dlx: installed package identity or executable metadata is invalid")
}
pub fn dlx_bin_target_mismatch() -> Error {
    anyhow!("dlx: executable does not match the requested package's declared target")
}

/// Container scaffolding cannot map a split web project to one runtime image.
/// Không thể gộp fullstack/monorepo nhiều runtime vào một image container.
pub fn web_container_runtime_split_unsupported() -> Error {
    anyhow!(
        "container scaffolding currently requires one runtime per web project; split fullstack/monorepo containers are not implemented"
    )
}

/// Container scaffolding currently has a verified Node web runtime only.
/// Lane container hiện chỉ xác minh runtime web Node.
pub fn web_container_node_lane_required() -> Error {
    anyhow!(
        "container scaffolding currently supports Node web projects only; non-Node web lanes are not implemented"
    )
}

/// A requested container action has no implemented MGC lifecycle profile.
/// Lệnh container chỉ được tạo khi có profile lifecycle MGC đã triển khai.
pub fn container_profile_unsupported(core: &str, framework: &str, runtime: &str) -> Error {
    anyhow!(
        "container scaffolding has no MGC lifecycle profile for core '{core}', framework '{framework}', runtime '{runtime}'"
    )
}

/// Kubernetes output is limited to the verified HTTP service lane.
/// Kubernetes chỉ được sinh cho lane dịch vụ HTTP đã xác minh.
pub fn k8s_profile_unsupported(core: &str) -> Error {
    anyhow!(
        "Kubernetes scaffolding is currently supported only for Web HTTP services; core '{core}' has no verified workload profile"
    )
}

/// A selected framework must match a native container manifest profile.
/// Framework phải khớp manifest container native được hỗ trợ.
pub fn container_manifest_required(core: &str, framework: &str, manifest: &str) -> Error {
    anyhow!(
        "Docker scaffolding for core '{core}' framework '{framework}' requires the supported project manifest '{manifest}'"
    )
}

/// A scaffold framework spec failed canonical parsing.
/// Framework spec scaffold không qua được bước phân tích tên chuẩn.
pub fn invalid_framework_spec(core: &str, framework: &str, detail: &str) -> Error {
    anyhow!("invalid {core} framework specification '{framework}': {detail}")
}

/// Invalid manifest source/target path — đường dẫn template không được thoát root.
pub fn template_materialization_path_invalid(path: &str) -> Error {
    anyhow!("Unsafe template materialization path: {:?}", path)
}

/// Bound manifest parsing allocation — giới hạn bộ nhớ trước parse manifest.
pub fn template_manifest_too_large(limit: u64) -> Error {
    anyhow!("Template manifest exceeds the {} byte limit", limit)
}

/// Publishing errors never echo registry bodies or credential-bearing URLs.
/// Không đưa response body hoặc URL có credential vào lỗi publish.
pub fn template_publish_status(status: u16) -> Error {
    if status == 409 {
        anyhow!("Template version already exists; bump --version")
    } else {
        anyhow!("Template publish failed with HTTP {}", status)
    }
}

pub fn template_publish_auth_required() -> Error {
    anyhow!("Template registry requires authentication; set MGC_NPM_TOKEN")
}
