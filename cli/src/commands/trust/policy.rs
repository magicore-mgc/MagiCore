//! Trust policy enforcement for CI/CD
//! Thực thi policy trust cho CI/CD

use anyhow::Result;
use mgc_lockfile::{VerificationStatus, verify_lockfile_with_trust};
use std::path::Path;

/// Policy mode — Chế độ policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyMode {
    /// Strict: require signed lockfile (fail on unsigned/tampered)
    /// Nghiêm ngặt: yêu cầu lockfile đã ký (fail nếu chưa ký/tampered)
    Strict,

    /// Warn: warn on unsigned, fail on tampered
    /// Cảnh báo: cảnh báo nếu chưa ký, fail nếu tampered
    Warn,

    /// Audit: log only, never fail
    /// Audit: chỉ log, không fail
    Audit,
}

/// Enforce trust policy — Thực thi trust policy
pub fn enforce_policy(lockfile_path: &Path, mode: PolicyMode) -> Result<()> {
    if !lockfile_path.exists() {
        return Ok(()); // No lockfile = nothing to enforce
    }

    let status = verify_lockfile_with_trust(lockfile_path, &project_trust_keys(lockfile_path)?)?;

    match (mode, &status) {
        // Strict mode: fail on unsigned or invalid
        (PolicyMode::Strict, VerificationStatus::Unsigned) => {
            anyhow::bail!(
                "POLICY VIOLATION (strict): Lockfile not signed. Run 'mgc trust sign' or set MGC_TRUST_POLICY=warn"
            );
        }
        (PolicyMode::Strict, VerificationStatus::Tampered(msg)) => {
            anyhow::bail!("POLICY VIOLATION (strict): Lockfile tampered: {}", msg);
        }
        (PolicyMode::Strict, VerificationStatus::InvalidSignature(msg)) => {
            anyhow::bail!("POLICY VIOLATION (strict): Invalid signature: {}", msg);
        }
        (PolicyMode::Strict, VerificationStatus::Valid) => {
            eprintln!("✓ Trust policy: Lockfile signature valid (strict mode)");
        }
        (PolicyMode::Strict, VerificationStatus::UntrustedKey(key_id)) => {
            anyhow::bail!(
                "POLICY VIOLATION (strict): Lockfile signer '{key_id}' is not in mgc.toml [trust].keys"
            );
        }

        // Warn mode: warn on unsigned, fail on tampered
        (PolicyMode::Warn, VerificationStatus::Unsigned) => {
            eprintln!("WARN: Trust policy: Lockfile not signed (warn mode)");
        }
        (PolicyMode::Warn, VerificationStatus::Tampered(msg)) => {
            anyhow::bail!("POLICY VIOLATION (warn): Lockfile tampered: {}", msg);
        }
        (PolicyMode::Warn, VerificationStatus::InvalidSignature(msg)) => {
            anyhow::bail!("POLICY VIOLATION (warn): Invalid signature: {}", msg);
        }
        (PolicyMode::Warn, VerificationStatus::Valid) => {
            eprintln!("✓ Trust policy: Lockfile signature valid (warn mode)");
        }
        (PolicyMode::Warn, VerificationStatus::UntrustedKey(key_id)) => {
            eprintln!(
                "WARN: Lockfile signature is valid, but signer '{key_id}' is not trusted by mgc.toml [trust].keys"
            );
        }

        // Audit mode: log only, never fail
        (PolicyMode::Audit, status) => {
            eprintln!("ℹ Trust policy audit: {:?}", status);
        }
    }

    Ok(())
}

/// Verify with the project's committed trust roots and selected policy.
/// Xác minh theo trust roots trong project và policy đang chọn.
pub fn verify_project_lockfile(lockfile_path: &Path) -> Result<VerificationStatus> {
    verify_lockfile_with_trust(lockfile_path, &project_trust_keys(lockfile_path)?)
        .map_err(Into::into)
}

fn project_trust_keys(lockfile_path: &Path) -> Result<Vec<String>> {
    let root = lockfile_path.parent().unwrap_or_else(|| Path::new("."));
    let project = mgc_config::project::ProjectConfig::load(root)?;
    Ok(project
        .and_then(|config| config.trust)
        .map(|trust| trust.keys)
        .unwrap_or_default())
}

/// Apply project policy in all installs; CI is unconditionally strict, while
/// local mode honors explicit env/project policy and otherwise warns.
/// (CI luôn strict; máy local theo config/env rõ ràng, mặc định cảnh báo.)
pub fn enforce_project_policy(lockfile_path: &Path) -> Result<()> {
    let root = lockfile_path.parent().unwrap_or_else(|| Path::new("."));
    let configured = mgc_config::project::ProjectConfig::load(root)?
        .and_then(|config| config.lock.and_then(|lock| lock.policy));
    let env_policy = std::env::var("MGC_TRUST_POLICY").ok();
    let mode = resolve_effective_mode(
        configured.as_deref(),
        env_policy.as_deref(),
        is_ci_environment(),
    );
    enforce_policy(lockfile_path, mode)
}

fn resolve_effective_mode(
    project_policy: Option<&str>,
    env_policy: Option<&str>,
    is_ci: bool,
) -> PolicyMode {
    // CI is a trust boundary: repository or job environment must not turn
    // required signature verification into warn/audit mode.
    // (CI là ranh giới tin cậy; config/env không được hạ kiểm tra chữ ký.)
    if is_ci {
        return PolicyMode::Strict;
    }

    match env_policy {
        Some("strict" | "require") => PolicyMode::Strict,
        Some("warn") => PolicyMode::Warn,
        Some("audit" | "off") => PolicyMode::Audit,
        _ => match project_policy.and_then(mgc_lockfile::policy::LockPolicyMode::parse) {
            Some(mgc_lockfile::policy::LockPolicyMode::Require) => PolicyMode::Strict,
            Some(mgc_lockfile::policy::LockPolicyMode::Warn) => PolicyMode::Warn,
            Some(mgc_lockfile::policy::LockPolicyMode::Off) => PolicyMode::Audit,
            None => PolicyMode::Warn,
        },
    }
}

/// Provider variables that are meaningful by their mere presence.
///
/// These are set once by their provider and carry no "off" value; a non-empty
/// value of any of them means the job is running inside that provider.
/// (Biến provider mà sự tồn tại đã đủ nghĩa — provider đặt một lần, không có giá
/// trị "off"; giá trị khác rỗng của bất kỳ biến nào nghĩa là đang chạy trong
/// provider đó.)
const CI_PROVIDER_MARKERS: &[&str] = &[
    "GITHUB_ACTIONS",
    "GITLAB_CI",
    "CIRCLECI",
    "JENKINS_URL",
    "TRAVIS",
];

/// Check whether the process runs inside a CI job.
///
/// `CI` is read by VALUE, not by presence: a developer shell or a tool that
/// exports `CI=false` / `CI=0` / `CI=` is not CI, and treating it as CI would
/// silently force strict signature policy in a local build. Conversely the
/// provider markers are presence-based because none of them has an "off" form.
/// (`CI` đọc theo GIÁ TRỊ, không theo sự tồn tại: shell dev hay tool export
/// `CI=false`/`CI=0`/`CI=` không phải CI — coi là CI sẽ ép policy strict trong
/// build local. Marker provider thì theo sự tồn tại vì chúng không có dạng
/// "off".)
pub fn is_ci_environment() -> bool {
    env_flag_enabled("CI")
        || CI_PROVIDER_MARKERS
            .iter()
            .any(|name| env_present_and_non_empty(name))
}

/// A boolean-ish environment flag is enabled unless it says otherwise.
///
/// `1`/`true`/`yes`/`on` (any case) are on; `0`/`false`/`no`/`off` and an empty
/// value are off; any other non-empty value is treated as on, matching how CI
/// providers set `CI=1`.
/// (Flag boolean: `1/true/yes/on` là bật; `0/false/no/off` và rỗng là tắt; giá
/// trị khác rỗng coi là bật — đúng cách CI provider đặt `CI=1`.)
fn env_flag_enabled(name: &str) -> bool {
    match std::env::var(name) {
        Ok(value) => {
            let value = value.trim();
            if value.is_empty() {
                return false;
            }
            !matches!(
                value.to_ascii_lowercase().as_str(),
                "0" | "false" | "no" | "off"
            )
        }
        Err(_) => false,
    }
}

/// Whether the variable exists and carries a non-empty value.
/// Biến tồn tại và có giá trị khác rỗng.
fn env_present_and_non_empty(name: &str) -> bool {
    std::env::var(name).is_ok_and(|value| !value.trim().is_empty())
}

#[cfg(test)]
#[path = "test/trust_policy.rs"]
mod trust_policy_tests;
