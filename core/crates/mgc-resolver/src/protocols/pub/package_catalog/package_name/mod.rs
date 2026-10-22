//! Package identity owner wiring.
//! Nối owner định danh package.

mod package_name;
pub(in crate::protocols::pubdev) use package_name::validate_pub_package_name;
