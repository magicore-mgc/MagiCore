//! Declared Flutter SDK dependency parser owner wiring.
//! Wiring bộ đọc dependency SDK Flutter được khai báo.

mod declared_dependencies;
pub(in crate::manifest::flutter::flutter::sdk) use declared_dependencies::declared_flutter_sdk_dependencies;
