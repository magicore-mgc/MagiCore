//! Shared framework route DTO and evidence text — DTO và evidence framework dùng chung.

/// Route category for the framework catalog. Lifecycle evidence is reported
/// per record, and dependency ownership is reported separately per operation.
/// (Phân loại route của catalog; E2E và quyền dependency được báo riêng.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameworkStatus {
    /// Offered by the wizard without an explicit `mgc-engine-path` route label.
    /// This label does not negate lifecycle evidence in the record.
    /// (Wizard có lựa chọn nhưng không mang nhãn route engine tường minh;
    /// bằng chứng lifecycle vẫn nằm trong record.)
    ScaffoldOnly,
    /// The framework branch routes through an MGC-owned engine path; this
    /// alone is not a claim that every dependency operation is supported.
    NativeEngine,
}

/// One qualification record: (core, wizard framework id) → claim.
/// (Một record qualification.)
#[derive(Clone, Copy)]
pub struct FrameworkRecord {
    pub core: &'static str,
    pub framework: &'static str,
    pub status: FrameworkStatus,
    pub evidence: &'static str,
}

/// Shared evidence pointers (precise test/lane paths, no hand-waving).
/// (Căn cứ chung.)
pub const EV_WIZARD: &str = "wizard answer id only (lifecycle unqualified beyond scaffold)";
pub const EV_SCAFFOLD_WEB: &str =
    "cli/tests/web_fw_* + templates/backend.rs (scaffold proven, lifecycle unqualified)";
pub const EV_DEPGATE: &str =
    "dep_gate owner table + cli/tests/dep_gate_canary (zero-spawn proof per lane)";
pub const EV_LIB_PIPELINE: &str =
    "adapters/lib/src/install (spawn-free scan) + mgc-resolver protocols_* tests + dep_gate table";
pub const EV_WEB_ENGINE: &str = "MGC-owned JS web dependency pipeline (resolve/lock/fetch/CAS/materialize) + dep_gate web Native cell; native lifecycle E2E for all 18 JS web frameworks on macOS arm64 in debug and release profiles (`cli/tests/web_fw_native_matrix.rs`, 11 lanes; `cli/tests/web_fw_fe_*.rs`, 7 lanes), covering install/frozen replay/test/build when defined/dev HTTP readiness/process and port cleanup. Dependency ownership remains reported per operation.";
pub const EV_RN_GATE: &str =
    "dep_gate app/rn Unsupported cell + dep_gate_canary rn_* (no runner exists)";

/// Evidence from prior E2E runs. Historical spawn evidence does not prove
/// current dependency ownership; operation cells below come from dep_gate.
/// (E2E lịch sử không chứng minh ownership hiện tại; operation lấy từ dep_gate.)
pub const EV_E2E_APP_FLUTTER: &str = "Current native dependency E2E: `add-app fixture_pkg` resolves, verifies, writes pubspec+mgc.lock and installs the graph with zero Flutter/Dart spawn; Flutter list/dev and dependency-health outdated scan remain toolchain-owned. Historical delegated `flutter pub add` evidence is not current ownership proof.";
pub const EV_E2E_APP_SWIFT: &str = "Historical SwiftPM spawn is not current ownership evidence; see per-operation owner cells derived from dep_gate";
pub const EV_E2E_CLO_TERRAFORM: &str = "Terraform dependency operations are currently unsupported; deploy/tool execution is a separate capability and does not imply native package ownership";
pub const EV_E2E_HARDWARE: &str = "E2E template lane: create/add/install/list all exit 0 (optimizer.json/bench.json materialized); `mgc optimizer` applied profile.json + runtime envs on web+ai projects; `mgc bench` timed install; no package lifecycle by design";
pub const EV_E2E_IOT_PIO: &str = "Historical PlatformIO spawn is not current ownership evidence; current dependency operations are unsupported";
pub const EV_E2E_IOT_WEST: &str = "Historical West spawn is not current ownership evidence; current dependency operations are unsupported";
pub const EV_E2E_WEB_JS: &str = "Native JS web lifecycle E2E: 18 frameworks on macOS arm64 in debug and release profiles; `cli/tests/web_fw_native_matrix.rs` (11 lanes) + `cli/tests/web_fw_fe_*.rs` (7 lanes), covering install/frozen replay/test/build when defined/dev HTTP readiness/process and port cleanup. Dependency ownership remains reported per operation.";
pub const EV_E2E_WEB_BACKEND: &str = "Empty-graph install/frozen route E2E on V3 and V4 for all 12 advertised non-JS web backend frameworks (Actix/Axum, Django/FastAPI/Flask, Echo/Fiber/Gin, Quarkus/Spring Boot, .NET Minimal/Web API) plus five generic language aliases (Rust, Python, Go, Java, .NET); non-empty add/install/frozen replay against hermetic local registries is exercised for Python, Rust, Go, Java, and .NET on V3 and V4. Both paths preserve the .mgc.core web marker. Shared-store root routing is covered by a separate read-only store-doctor E2E. Web security-policy selection is covered separately by an adapter unit test. This does not qualify framework dev/build/test runtime.";
