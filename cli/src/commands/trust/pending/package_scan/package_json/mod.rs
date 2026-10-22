//! Package-manifest owner wiring.
//! Wiring owner manifest package.

mod package_json;
pub(super) use self::package_json::check_package_json;
