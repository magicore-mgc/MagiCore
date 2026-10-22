//! Wires package graph projection and its focused tests.
//! Nối phép chiếu đồ thị package và các test chuyên biệt.

mod package_conversion;
pub(super) use self::package_conversion::convert_packages;
#[cfg(test)]
mod test;
