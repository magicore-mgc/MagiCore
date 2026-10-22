//! Wires package-name validation and nearest-owner tests.
//! Nối xác thực tên package và test gần owner.

mod package_name;
pub use self::package_name::PackageName;
#[cfg(test)]
mod test;
