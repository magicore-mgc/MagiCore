//! Wires safety-backup path planning and preparation.
//! Nối phần lập kế hoạch và chuẩn bị safety-backup path.

mod safety_destination;

pub(super) use safety_destination::prepare_safety_backup;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
