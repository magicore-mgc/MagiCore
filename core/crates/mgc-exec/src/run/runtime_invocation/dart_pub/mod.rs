//! Dart and Flutter package-command parser exports.
//! Export parser lệnh package Dart và Flutter.

#[path = "dart_pub.rs"]
mod implementation;
pub(super) use implementation::dart_pub_argument;
