//! Serial-port detection owner wiring.
//! Wiring owner nhận diện cổng serial.

mod serial_port_detection;

pub use serial_port_detection::{detect_serial_ports, detect_serial_ports_in};
