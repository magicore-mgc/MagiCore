//! Read Swift resolved lockfile owner wiring.
//! File này chỉ nối module parser Package.resolved và export API công khai.

mod read_swift_resolved;
pub use read_swift_resolved::read_swift_resolved;
