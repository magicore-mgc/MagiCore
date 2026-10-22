//! Stable facade for App Swift manifest operations.
//! Facade ổn định cho thao tác manifest Swift của App.

mod package_source;
mod resolved_pins;
mod swift;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub use self::package_source::scan_package_swift_text;
pub use self::swift::{parse_package_swift, write_package_swift};
