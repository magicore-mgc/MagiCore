//! Locate Swift resolved lockfile owner wiring.
//! File này chỉ nối module tìm Package.resolved và export API công khai.

mod swift_resolved_path;
pub use swift_resolved_path::swift_resolved_path;

#[cfg(test)]
#[path = "test/swift_resolved_path.rs"]
mod tests;
