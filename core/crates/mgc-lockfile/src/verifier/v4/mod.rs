//! Wires v4 inline-signature verification.
//! Nối xác minh chữ ký inline của v4.

mod v4;

pub(in crate::verifier) use self::v4::verify_v4_lockfile;
