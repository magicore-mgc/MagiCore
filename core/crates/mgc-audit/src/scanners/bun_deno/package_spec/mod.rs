//! Package spec splitter owner — owner tách package spec.
//! This pure helper is shared by Bun and Deno readers.
//! Helper thuần này được dùng chung bởi bộ đọc Bun và Deno.

mod package_spec;

pub(super) use package_spec::split_name_version;
