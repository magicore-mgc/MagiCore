//! Identify the supported MagiCore ecosystems.
//! Định danh các ecosystem mà MagiCore hỗ trợ.

/// Stable identifier shared by manifests, adapters and plugin routing.
/// Định danh ổn định dùng chung cho manifest, adapter và plugin routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Ecosystem {
    Web,
    Game,
    Ai,
    Cloud,
    Cicd,
    Iot,
    App,
    Lib,
    Hardware,
}

/// The single canonical CLI identity of the Cloud ecosystem.
///
/// `clo` wins because it is what the user types (`mgc create-clo`,
/// `mgc install-clo`, `--core clo`), what the core marker stores, and what
/// `CoreIdent::core_id()` returns. Every other spelling (`cloud` in
/// `adapters/cloud`, template namespaces and registry package names) is a
/// PASSTHROUGH namespace that must convert through this constant instead of
/// carrying its own alias table.
/// (Định danh CLI chuẩn duy nhất của ecosystem Cloud. `clo` thắng vì đó là thứ
/// người dùng gõ, là giá trị marker lưu và là giá trị `core_id()` trả. Mọi cách
/// viết khác chỉ là namespace passthrough, phải đi qua hằng này.)
const CLOUD_CANONICAL: &str = "clo";

/// The accepted input alias for Cloud (host tag / directory name).
/// Bí danh đầu vào được chấp nhận cho Cloud (tag host / tên thư mục).
const CLOUD_INPUT_ALIAS: &str = "cloud";

impl Ecosystem {
    /// Every ecosystem, in the fixed order used by `as_str`/`iter`.
    /// Mọi ecosystem, theo thứ tự cố định mà `as_str`/`iter` dùng.
    pub const ALL: [Ecosystem; 9] = [
        Self::Web,
        Self::Game,
        Self::Ai,
        Self::Cloud,
        Self::Cicd,
        Self::Iot,
        Self::App,
        Self::Lib,
        Self::Hardware,
    ];

    /// Parse a canonical core name or the accepted `cloud` input alias.
    /// Parse tên core chuẩn hoặc bí danh đầu vào `cloud` được chấp nhận.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "web" => Some(Self::Web),
            "game" => Some(Self::Game),
            "ai" => Some(Self::Ai),
            "clo" | CLOUD_INPUT_ALIAS => Some(Self::Cloud),
            "cicd" => Some(Self::Cicd),
            "iot" => Some(Self::Iot),
            "app" => Some(Self::App),
            "lib" => Some(Self::Lib),
            "hardware" => Some(Self::Hardware),
            _ => None,
        }
    }

    /// Parse an ecosystem value from project metadata (marker / `mgc.toml`).
    ///
    /// Unlike [`Self::from_str`] this never accepts the historical input alias:
    /// a persisted `ecosystem = "cloud"` written by an older build is read as
    /// the same ecosystem, but callers get a canonical value back, so a new
    /// compensating alias table is never needed again.
    /// (Parse giá trị ecosystem từ metadata project. Khác `from_str`, hàm này
    /// không bao giờ trả về alias: giá trị `"cloud"` do bản cũ ghi vẫn đọc được
    /// nhưng caller nhận giá trị chuẩn.)
    pub fn from_persisted_str(value: &str) -> Option<Self> {
        let value = value.trim();
        if value.eq_ignore_ascii_case(CLOUD_CANONICAL) {
            return Some(Self::Cloud);
        }
        Self::from_str(&value.to_ascii_lowercase())
    }

    /// Canonical CLI identity: always equal to `CoreIdent::core_id()`.
    /// Định danh CLI chuẩn: luôn bằng `CoreIdent::core_id()`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Web => "web",
            Self::Game => "game",
            Self::Ai => "ai",
            Self::Cloud => CLOUD_CANONICAL,
            Self::Cicd => "cicd",
            Self::Iot => "iot",
            Self::App => "app",
            Self::Lib => "lib",
            Self::Hardware => "hardware",
        }
    }

    /// Convert an already-lowercase core name to the canonical spelling.
    ///
    /// This is the ONE alias table in the workspace: it replaces the
    /// `if name == "cloud"` copies that used to live in the marker writer,
    /// the lockfile owners and the import lane.
    /// (Chuyển tên core đã lowercase về cách viết chuẩn. Đây là BẢNG ALIAS DUY
    /// NHẤT trong workspace, thay các bản copy `if name == "cloud"` trước đây.)
    pub fn canonical_core_name(value: &str) -> String {
        Self::from_str(value)
            .map(|ecosystem| ecosystem.as_str().to_string())
            .unwrap_or_else(|| value.to_string())
    }

    /// Passthrough namespaces that carry a real host tag for this core.
    ///
    /// These are NOT aliases for project identity: `cloud` is the adapter
    /// package name, the template layer directory (pinned by the published
    /// template registry) and the npm package namespace
    /// (`mgc-create-cloud-*`). A caller that needs one of those must ask here
    /// by role, so adding a namespace is a deliberate act rather than a new
    /// local `match`.
    /// (Namespace passthrough mang tag host thật: tên package adapter, thư mục
    /// layer template và namespace npm. Caller cần thì hỏi theo vai trò.)
    pub fn passthrough_namespace(&self, role: NamespaceRole) -> &'static str {
        match (self, role) {
            (Self::Cloud, NamespaceRole::AdapterDirectory) => CLOUD_INPUT_ALIAS,
            (Self::Cloud, NamespaceRole::TemplateLayer) => CLOUD_INPUT_ALIAS,
            (Self::Cloud, NamespaceRole::RegistryPackage) => CLOUD_INPUT_ALIAS,
            _ => self.as_str(),
        }
    }
}

/// The external namespace a core name is needed in.
/// Namespace bên ngoài mà tên core cần cho nó.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceRole {
    /// Adapter package / adapter directory (`adapters/<name>`).
    /// Package/thư mục adapter.
    AdapterDirectory,
    /// First segment of a template layer path (`templates/<name>/**`).
    /// Segment đầu của path layer template.
    TemplateLayer,
    /// npm registry package namespace (`mgc-create-<name>-*`).
    /// Namespace package trên npm registry.
    RegistryPackage,
}
