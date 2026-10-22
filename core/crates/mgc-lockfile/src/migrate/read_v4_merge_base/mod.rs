//! Wires reads the existing v4 base and reports leftover legacy rows.
//! Nối owner: Đọc base v4 hiện có và báo row legacy còn lại.

mod read_v4_merge_base;
pub use self::read_v4_merge_base::read_v4_merge_base;
#[cfg(test)]
mod test;
