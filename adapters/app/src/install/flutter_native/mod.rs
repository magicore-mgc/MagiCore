//! Flutter-native install owner.
//! Module chủ quản cài Flutter native.
mod flutter_native;
pub(crate) use flutter_native::install_flutter_native;
#[cfg(test)]
pub(crate) use flutter_native::preflight_flutter_archives;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
