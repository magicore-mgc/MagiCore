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
            CoreKind::Cloud => "cloud",
            CoreKind::Hardware => "hardware",
        }
    }

    /// Parse an established lowercase core name.
    /// Parse tên core viết thường hiện hành.
    pub fn from_str_core(s: &str) -> Option<Self> {
        match s {
            "web" => Some(CoreKind::Web),
            "ai" => Some(CoreKind::Ai),
            "app" => Some(CoreKind::App),
            "lib" => Some(CoreKind::Lib),
            "game" => Some(CoreKind::Game),
            "iot" => Some(CoreKind::Iot),
            "cicd" => Some(CoreKind::Cicd),
            "cloud" => Some(CoreKind::Cloud),
            "hardware" => Some(CoreKind::Hardware),
            _ => None,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
