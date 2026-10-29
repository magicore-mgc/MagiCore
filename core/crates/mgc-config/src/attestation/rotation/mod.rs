//! rotation owner wiring.
//! Wiring owner Giữ một thế hệ key ký cũ khi key thay đổi.
mod rotation;
pub(super) use rotation::previous_key_for;
pub(crate) use rotation::{now_rfc3339, rotation_key_expired};
