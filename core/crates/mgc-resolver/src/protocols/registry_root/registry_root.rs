//! Root requirements with registry-specific feature selections.
//! Root requirement cùng lựa chọn feature riêng theo registry.

/// Project root requirement with registry-specific feature selection.
/// Root requirement của project kèm lựa chọn feature riêng cho registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryRoot {
    pub name: String,
    pub range: String,
    pub features: Vec<String>,
    pub default_features: bool,
}
