//! Gradle and Maven offline policy facade.
//! Facade policy offline Gradle và Maven.

#[path = "java.rs"]
mod implementation;
pub(super) use implementation::denial_reason;
