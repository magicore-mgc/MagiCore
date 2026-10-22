//! Keeps the public package-domain API stable and wires focused owners.
//! Giữ API miền package ổn định và nối các owner chuyên trách.

mod dependency_spec;
mod package_id;
mod package_name;
mod version_range;

pub use self::dependency_spec::DependencySpec;
pub use self::package_id::PackageId;
pub use self::package_name::PackageName;
pub use self::version_range::VersionRange;
