//! Options for the add operation, shared by every adapter core. — Tùy chọn thao tác add dùng chung cho mọi adapter core.

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AddOptions {
    pub dev: bool,
    pub optional: bool,
    pub peer: bool,
    pub exact: bool,
    pub no_save: bool,
    pub global: bool,
}
