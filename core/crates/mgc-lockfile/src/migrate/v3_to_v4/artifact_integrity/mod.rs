//! Wires digest-preserving artifact conversion and its tests.
//! Nối chuyển đổi artifact giữ nguyên digest và test.

mod artifact_integrity;
pub(super) use self::artifact_integrity::carry_install_sri;
#[cfg(test)]
mod test;
