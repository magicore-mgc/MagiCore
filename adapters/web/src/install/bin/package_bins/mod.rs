//! Wiring for package bin-target validation and link materialization.
//! Wiring cho xác thực target bin và tạo link package.

mod package_bins;
pub(super) use package_bins::materialize_package_bins;
