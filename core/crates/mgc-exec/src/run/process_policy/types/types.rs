//! Cross-platform description of a matched forbidden child.
//! Mô tả child bị cấm đã match, dùng chung nhiều nền tảng.

#[derive(Debug, Clone)]
pub(in crate::run::implementation) struct ForbiddenProcess {
    pub(in crate::run::implementation) pid: u32,
    pub(in crate::run::implementation) name: String,
    pub(in crate::run::implementation) image_name: Option<String>,
    pub(in crate::run::implementation) match_source: &'static str,
    pub(in crate::run::implementation) matched_token: Option<String>,
    pub(in crate::run::implementation) matched_token_index: Option<usize>,
    pub(in crate::run::implementation) matched_operation: String,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
