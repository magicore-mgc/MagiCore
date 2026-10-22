//! Format a per-core command name — dựng tên command theo core.
//! Preserve the established `{prefix}-{core}` spelling.
//! Giữ cách viết hiện hành `{prefix}-{core}`.

pub(in crate::help::sections) fn per_core_name(prefix: &str, core: &str) -> String {
    format!("{prefix}-{core}")
}
