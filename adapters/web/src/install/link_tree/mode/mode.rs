//! Select strict package-link strategy from the Web runtime environment.
//! Chọn cách liên kết package strict theo environment của Web runtime.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrictTreeLinkMode {
    Symlink,
    Hardlink,
}

pub fn strict_tree_link_mode() -> StrictTreeLinkMode {
    match std::env::var("MAGICORE_WEB_STRICT_TREE_MODE")
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "hardlink" | "copy" | "compat" => StrictTreeLinkMode::Hardlink,
        "symlink" | "link" | "fast" => StrictTreeLinkMode::Symlink,
        _ => StrictTreeLinkMode::Hardlink,
    }
}
