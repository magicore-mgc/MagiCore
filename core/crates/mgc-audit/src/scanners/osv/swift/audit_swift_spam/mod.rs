//! Audit Swift Package Manager owner wiring.
//! File này chỉ nối module audit Swift và export API công khai.

mod audit_swift_spam;
pub use audit_swift_spam::audit_swift_spam;
