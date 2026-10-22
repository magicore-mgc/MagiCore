//! IoT flash boundary wiring and stable API exports.
//! Wiring ranh giới flash IoT và export API ổn định.

mod flash;
mod flash_result;
mod serial_port_detection;

pub use flash::flash_firmware;
pub use flash_result::FlashResult;
pub use serial_port_detection::{detect_serial_ports, detect_serial_ports_in};

#[cfg(test)]
#[path = "test/flash_tests.rs"]
mod tests;
