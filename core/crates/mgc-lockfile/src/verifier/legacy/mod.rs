//! Wires legacy sidecar verification and outcome mapping.
//! Nối xác minh sidecar legacy và ánh xạ kết quả.

mod legacy;

pub(in crate::verifier) use self::legacy::verify_legacy_lockfile;
