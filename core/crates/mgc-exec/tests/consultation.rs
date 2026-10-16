//! Live exception consultation tests — kiểm thử consultation exception live.
//!
//! The exec pre-spawn layer consults the verified manifest supplied in
//! `ExecOptions` (Spec rev4 §5/§7): MGC-answered probes return without
//! spawning; exact read-only manifest exceptions require a matching active
//! profile and never bypass dependency-mutation checks.
//! (Lớp pre-spawn exec tra manifest đã xác minh trong `ExecOptions`
//! (Spec rev4 §5/§7): probe MGC tự trả lời trả về không spawn; exception
//! manifest chỉ được dùng với profile hiệu lực và không thể bỏ qua mutation.)

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use mgc_exec::policy::{LaneProfile, PolicyContext, ReadOnlyException};
use mgc_exec::policy_manifest::{ManifestEntry, POLICY_MANIFEST_SCHEMA, PolicyManifest};
use mgc_exec::prelude::{ExecOptions, run};

fn context() -> PolicyContext {
    PolicyContext {
        framework: "test-fw".to_string(),
        version: String::new(),
        os: std::env::consts::OS.to_string(),
        operation: "exec".to_string(),
    }
}

fn manifest_with(entry: ManifestEntry) -> PolicyManifest {
    PolicyManifest {
        schema_version: POLICY_MANIFEST_SCHEMA.to_string(),
        id: "test".to_string(),
        issued_at: 1,
        expires_at: 9_000_000_000,
        key_id: "test-key".to_string(),
        revoked_keys: vec![],
        entries: vec![entry],
        signature: "test-sig".to_string(),
    }
}

fn opts_with(manifest: PolicyManifest) -> ExecOptions {
    ExecOptions {
        policy_manifest: Some(manifest),
        policy_context: Some(context()),
        policy_profile: Some(LaneProfile {
            framework: "test-fw".to_string(),
            version: String::new(),
            os: std::env::consts::OS.to_string(),
            operation: "exec".to_string(),
            allowed_pm_invocations: vec!["pnpm --version".to_string()],
            requires_pm: vec![],
        }),
        ..Default::default()
    }
}

// MGC-answered probes return without spawning anything.
// (Probe MGC tự trả lời trả về không spawn gì.)
#[test]
fn self_answer_returns_without_spawning() {
    let opts = ExecOptions::default();
    let report = run("npm", &["--version".to_string()], &opts).unwrap();
    assert_eq!(report.exit_code, 0);
    assert_eq!(report.stdout_tail, "0.0.0");
}

// An exact profile-bound, read-only PM probe may pass the name gate.
// (Probe PM chỉ đọc, khớp profile và manifest, có thể qua name gate.)
#[test]
#[cfg(unix)]
fn exact_exception_permits_matching_probe() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::TempDir::new().unwrap();
    let fake = dir.path().join("pnpm");
    std::fs::write(&fake, "#!/bin/sh\nexit 0\n").unwrap();
    let mut permissions = std::fs::metadata(&fake).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&fake, permissions).unwrap();

    let entry = ManifestEntry {
        program: "pnpm".to_string(),
        args_exact: vec!["--version".to_string()],
        framework: "test-fw".to_string(),
        framework_version: String::new(),
        os: std::env::consts::OS.to_string(),
        operation: "exec".to_string(),
        reason: "test probe".to_string(),
        expected: "exit 0".to_string(),
    };
    let report = run(
        fake.to_str().unwrap(),
        &["--version".to_string()],
        &opts_with(manifest_with(entry)),
    )
    .unwrap();
    assert_eq!(report.exit_code, 0);
}

// A signed exact probe without an active lane profile remains denied.
// Probe có manifest khớp nhưng thiếu profile lane vẫn bị từ chối.
#[test]
#[cfg(unix)]
fn exception_without_profile_stays_denied() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::TempDir::new().unwrap();
    let pnpm = dir.path().join("pnpm");
    std::fs::write(&pnpm, "#!/bin/sh\nexit 0\n").unwrap();
    let mut permissions = std::fs::metadata(&pnpm).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&pnpm, permissions).unwrap();
    let entry = ManifestEntry {
        program: "pnpm".to_string(),
        args_exact: vec!["--version".to_string()],
        framework: "test-fw".to_string(),
        framework_version: String::new(),
        os: std::env::consts::OS.to_string(),
        operation: "exec".to_string(),
        reason: "test probe".to_string(),
        expected: "exit 0".to_string(),
    };
    let opts = ExecOptions {
        policy_manifest: Some(manifest_with(entry)),
        policy_context: Some(context()),
        ..Default::default()
    };
    let err = run(&pnpm.to_string_lossy(), &["--version".to_string()], &opts).unwrap_err();
    assert!(err.to_string().contains("forbidden"), "got: {err}");
}

// A mutation or executable-code verb cannot be converted into a read-only
// exception, even when it appears in both signed data and a profile.
// Mutation hoặc verb chạy code không thể thành read-only exception, kể cả
// khi được ghi trong manifest và profile.
#[test]
#[cfg(unix)]
fn mutation_and_exec_verbs_do_not_use_manifest_exception() {
    let entry = ManifestEntry {
        program: "npm".to_string(),
        args_exact: vec!["exec".to_string(), "evil".to_string()],
        framework: "test-fw".to_string(),
        framework_version: String::new(),
        os: std::env::consts::OS.to_string(),
        operation: "exec".to_string(),
        reason: "must not be used".to_string(),
        expected: "".to_string(),
    };
    let mut opts = opts_with(manifest_with(entry));
    opts.policy_profile
        .as_mut()
        .unwrap()
        .allowed_pm_invocations
        .push("npm exec evil".to_string());
    let err = run("npm", &["exec".to_string(), "evil".to_string()], &opts).unwrap_err();
    assert!(err.to_string().contains("forbidden"), "got: {err}");
}

// Without a manifest, the same fake binary stays denied (not allowlisted).
// (Không manifest, cùng binary giả vẫn bị deny (không trong allowlist).)
#[test]
#[cfg(unix)]
fn no_manifest_stays_denied() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::TempDir::new().unwrap();
    let fake = dir.path().join("fakepm");
    std::fs::write(&fake, "#!/bin/sh\nexit 0\n").unwrap();
    let mut permissions = std::fs::metadata(&fake).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&fake, permissions).unwrap();

    let opts = ExecOptions::default();
    let err = run(fake.to_str().unwrap(), &[], &opts).unwrap_err();
    assert!(
        err.to_string().contains("allowlist"),
        "unexpected error: {err}"
    );
}

// Context mismatch (wrong operation) means no exception applies.
// (Ngữ cảnh lệch (sai operation) nghĩa là không exception nào áp.)
#[test]
#[cfg(unix)]
fn context_mismatch_denies() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::TempDir::new().unwrap();
    let pnpm = dir.path().join("pnpm");
    std::fs::write(&pnpm, "#!/bin/sh\nexit 0\n").unwrap();
    let mut permissions = std::fs::metadata(&pnpm).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&pnpm, permissions).unwrap();

    let entry = ManifestEntry {
        program: "pnpm".to_string(),
        args_exact: vec!["--version".to_string()],
        framework: "test-fw".to_string(),
        framework_version: String::new(),
        os: std::env::consts::OS.to_string(),
        operation: "dev".to_string(),
        reason: "test probe".to_string(),
        expected: "exit 0".to_string(),
    };
    let manifest = manifest_with(entry);
    let mut ctx = context();
    ctx.operation = "exec".to_string();
    let mut opts = opts_with(manifest);
    opts.policy_context = Some(ctx);
    opts.policy_profile.as_mut().unwrap().allowed_pm_invocations =
        vec!["pnpm --version".to_string()];
    let opts = ExecOptions {
        policy_manifest: opts.policy_manifest,
        policy_context: opts.policy_context,
        policy_profile: opts.policy_profile,
        ..Default::default()
    };
    let err = run(&pnpm.to_string_lossy(), &["--version".to_string()], &opts).unwrap_err();
    assert!(err.to_string().contains("forbidden"), "got: {err}");
}

// The unused-import guard: ReadOnlyException stays part of the public shape.
// (Giữ public shape: ReadOnlyException vẫn dùng được.)
#[test]
fn exception_shape_compiles() {
    let _ = ReadOnlyException {
        program: "npm".to_string(),
        args_exact: vec!["--version".to_string()],
        unexpired: true,
        answer: None,
    };
}
