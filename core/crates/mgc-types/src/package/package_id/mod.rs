//! Wires package identity and nearest-owner tests.
//! Nối identity package và test gần owner.

mod package_id;
pub use self::package_id::PackageId;
#[cfg(test)]
mod test;
