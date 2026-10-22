//! Wires checks v4 digest consistency before preserving sibling rows.
//! Nối owner: Kiểm tra tính nhất quán digest v4 trước khi giữ row sibling.

mod verify_v4_merge_base;
pub use self::verify_v4_merge_base::verify_v4_merge_base;
#[cfg(test)]
mod test;
