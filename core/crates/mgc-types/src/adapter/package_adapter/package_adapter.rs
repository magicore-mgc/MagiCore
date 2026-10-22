//! The stable PackageAdapter façade trait. Its capability supertraits, async method set, default behavior and refusal messages are kept unchanged. — Trait façade PackageAdapter ổn định. Capability supertrait, tập async method, default behavior và thông báo từ chối được giữ nguyên.

use std::path::Path;

use async_trait::async_trait;

use crate::error::{MgError, MgResult};
use crate::manifest::Manifest;
use crate::package::{PackageId, PackageName, VersionRange};

use super::super::{AddOptions, InstalledPackage, ManifestIdentity, PreparedAdd};
/// PackageAdapter — the FAÇADE combining the capability traits (Global
/// Gate 1, phase 1). CLI dispatch keeps calling these methods unchanged;
/// capability methods live on the supertraits in `crate::capabilities`
/// with fail-closed defaults, and per-adapter impls are split per trait.
/// PackageAdapter — FAÇADE kết hợp các capability trait (Global Gate 1,
/// phase 1). Dispatch CLI giữ nguyên cách gọi; method capability nằm trên
/// các supertrait trong `crate::capabilities` với default fail-closed,
/// impl từng adapter được tách theo trait.
#[async_trait]
pub trait PackageAdapter:
    crate::capabilities::CoreIdent
    + crate::capabilities::ProjectDetector
    + crate::capabilities::DependencyResolver
    + crate::capabilities::ArtifactFetcher
    + crate::capabilities::ContentStoreProvider
    + crate::capabilities::LockfileProvider
    + crate::capabilities::AuditProvider
    + crate::capabilities::ScaffoldProvider
    + crate::capabilities::LifecycleRunner
    + crate::capabilities::OptimizerProvider
    + crate::capabilities::Materializer
    + crate::capabilities::SimulatorProvider
    + crate::capabilities::DeviceProvider
    + crate::capabilities::DeployProvider
    + crate::capabilities::ModelRuntimeProvider
    + Send
    + Sync
{
    /// Orchestrator-level: parse the project manifest (not a capability —
    /// every adapter needs it for mixed flows). Parse manifest project —
    /// cấp orchestrator (không phải capability — mọi adapter cần cho flow
    /// trộn).
    async fn parse_manifest(&self, project_root: &Path) -> MgResult<Manifest>;

    /// Whether resolving an otherwise-empty manifest may still produce
    /// dependencies owned implicitly by this ecosystem (for example, Pub
    /// registry dependencies declared by Flutter SDK packages).
    /// (Cho biết manifest rỗng vẫn có thể resolve dependency ngầm do ecosystem sở hữu.)
    fn resolves_implicit_dependencies(&self, _project_root: &Path) -> MgResult<bool> {
        Ok(false)
    }

    /// Effective dependency requirements for lock replay, including SDK-owned registry roots.
    /// Tập dependency để replay lock; không dùng bản mở rộng này để ghi manifest project.
    fn lock_replay_manifest(
        &self,
        _project_root: &Path,
        manifest: &Manifest,
    ) -> MgResult<Manifest> {
        Ok(manifest.clone())
    }

    /// Orchestrator-level: list installed packages. Liệt kê package đã cài
    /// — cấp orchestrator.
    async fn list(&self, project_root: &Path) -> MgResult<Vec<InstalledPackage>>;

    /// Does MGC own this adapter's manifest file? `false` means dependency
    /// mutations must fail closed; there is no provider-tool fallback.
    /// Owned writes are committed by the CLI mutation gateway, not by direct
    /// adapter `add/remove/update` methods. Default `true`.
    /// (MGC có sở hữu manifest không? `false` thì mutation phải fail-closed,
    /// không fallback tool ngoài. Ghi manifest qua CLI mutation gateway.)
    fn manifest_owned(&self) -> bool {
        true
    }

    /// Can mgc-native update run on this adapter (resolve-latest +
    /// mgc-side manifest edit + native install, zero toolchain spawn)?
    /// `true` requires prepare_add + a round-tripping writer for every
    /// savable range. Default `false` (legacy adapter.update path).
    /// (Adapter này update native được không? Mặc định không.)
    fn supports_native_update(&self) -> bool {
        false
    }

    /// Arm this operation's security policy from its project root
    /// (P0/F6). Called by orchestrators BEFORE resolve on every
    /// operation that knows its project. Default: no-op (cores without
    /// per-project gates keep compiling and behaving identically).
    /// (Nạp policy bảo mật của operation từ root project của nó —
    /// mặc định không làm gì.)
    fn arm_age_gate_for(&self, _project_root: &Path) -> MgResult<()> {
        Ok(())
    }

    /// Whether this adapter can enforce the configured minimum-release-age
    /// policy for this project. CLI install rejects explicit policies when
    /// this returns false instead of silently resolving without a gate.
    /// Adapter này có thực thi được minimum-release-age cho project không;
    /// CLI từ chối policy tường minh nếu false để không resolve âm thầm.
    fn supports_age_gate_for(&self, _project_root: &Path) -> MgResult<bool> {
        Ok(false)
    }

    async fn prepare_add(
        &self,
        _project_root: &Path,
        name: &PackageName,
        range: Option<&VersionRange>,
        _opts: AddOptions,
    ) -> MgResult<PreparedAdd> {
        let _ = (name, range);
        Err(crate::capabilities::unsupported_capability(
            self.core_id(),
            "native prepare-add",
            "this adapter has no MagiCore-owned add resolver; provider package-manager fallback is disabled",
        ))
    }

    /// T5 audit --fix: re-resolve the given vulnerable packages to a newer
    /// version and rewrite the lockfile ONLY when re-resolution succeeds
    /// (fail-closed — a failed resolve leaves manifest + lockfile untouched).
    /// Returns the number of packages bumped. Default: unsupported — names
    /// the core so callers know what to ask for (R5).
    async fn audit_fix(&self, _project_root: &Path, _vulnerable: &[PackageId]) -> MgResult<usize> {
        Err(MgError::Other(format!(
            "audit --fix is not supported for the '{}' core — bump the vulnerable packages manually and re-run `mgc audit`",
            self.core_id(),
        )))
    }

    /// Enable dedupe preference (reuse installed versions) for the next resolve.
    /// Default no-op — opt-in per install (02 §2.1).
    fn set_dedupe_pref(&self, _enabled: bool) {}

    /// Provide already-installed versions (from lockfile) for dedupe resolution.
    /// Default no-op.
    fn set_existing_versions(&self, _versions: std::collections::HashMap<String, String>) {}

    /// Capability manifest — default claims NOTHING. Every adapter overrides
    /// this with its real `CAPABILITIES` const; the CLI exposes it via
    /// `mgc capabilities --json` and the lifecycle matrix reads it.
    /// Bảng capability — mặc định KHÔNG claim gì. Mỗi adapter override bằng
    /// const `CAPABILITIES` thật; CLI expose qua `mgc capabilities --json`
    /// và lifecycle matrix đọc từ đó.
    fn capabilities(&self) -> &'static [crate::capabilities::Capability] {
        &[]
    }

    /// Stable manifest identity for journals and crash recovery: which
    /// core, language/framework, manifest format and relative path this
    /// adapter reads and writes (e.g. lib/python/pyproject.toml). The
    /// default is None — NO native mutation ownership claimed. Recovery
    /// restores a journal ONLY through an identical identity: one lane
    /// must never rewrite another lane's manifest, even inside the same
    /// adapter (bevy vs godot, esp32 vs pio, tf vs cdk).
    /// `None` means this adapter owns no natively-mutable manifest;
    /// staging a journal for it fails closed.
    /// (Định danh manifest chi tiết — mặc định không claim gì.)
    fn manifest_identity(&self) -> Option<ManifestIdentity> {
        None
    }
}
