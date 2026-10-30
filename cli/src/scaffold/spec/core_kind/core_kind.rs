//! Stable core names used in scaffold requests.
//! Tên core ổn định dùng trong yêu cầu scaffold.

/// Core kind.
/// Loại core.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreKind {
    Web,
    Ai,
    App,
    Lib,
    Game,
    Iot,
    Cicd,
    Cloud,
    Hardware,
}

impl CoreKind {
    /// Return the established lowercase name for this core.
    /// Trả về tên viết thường hiện hành của core.
    pub fn as_str(&self) -> &'static str {
        match self {
            CoreKind::Web => "web",
            CoreKind::Ai => "ai",
            CoreKind::App => "app",
            CoreKind::Lib => "lib",
            CoreKind::Game => "game",
            CoreKind::Iot => "iot",
            CoreKind::Cicd => "cicd",
            CoreKind::Cloud => "clo",
            CoreKind::Hardware => "hardware",
        }
    }

    /// Parse an established lowercase core name OR a template/registry layer
    /// namespace, and return the core kind.
    ///
    /// `as_str()` is the CLI identity (`clo`), so the reverse parse accepts it.
    /// The layer namespace (`cloud/`) is NOT an identity: it reaches this parser
    /// from template paths that name a published directory, so it is resolved
    /// through the one alias table and must never be returned as `as_str()`.
    /// (Parse tên chuẩn hoặc namespace layer template. Namespace `cloud/` không
    /// phải identity — nó được resolve qua bảng alias duy nhất.)
    pub fn from_str_core(s: &str) -> Option<Self> {
        match s {
            "web" => Some(CoreKind::Web),
            "ai" => Some(CoreKind::Ai),
            "app" => Some(CoreKind::App),
            "lib" => Some(CoreKind::Lib),
            "game" => Some(CoreKind::Game),
            "iot" => Some(CoreKind::Iot),
            "cicd" => Some(CoreKind::Cicd),
            // `clo` is the identity; `cloud` is the published layer namespace
            // that `mgc create-clo` fetches. Both name the same core kind.
            // (`clo` là identity; `cloud` là namespace layer đã publish — cùng kind.)
            "clo" | "cloud" => Some(CoreKind::Cloud),
            "hardware" => Some(CoreKind::Hardware),
            _ => None,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
