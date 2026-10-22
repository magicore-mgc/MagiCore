//! Dart and Flutter package/build policy facade.
//! Facade policy package/build Dart và Flutter.

#[path = "flutter.rs"]
mod implementation;
pub(super) use implementation::denial_reason;
