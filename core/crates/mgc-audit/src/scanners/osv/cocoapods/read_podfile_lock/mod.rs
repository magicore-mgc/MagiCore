//! Read CocoaPods lockfile owner wiring.
//! File này chỉ nối module parser Podfile.lock và export API công khai.

mod read_podfile_lock;
pub use read_podfile_lock::read_podfile_lock;
