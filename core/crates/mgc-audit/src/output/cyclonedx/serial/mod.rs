//! Wire deterministic CycloneDX serial generation.
//! Nối tạo serial CycloneDX có tính xác định.

#[path = "serial.rs"]
mod serial;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(super) use self::serial::stable_serial_hash;
