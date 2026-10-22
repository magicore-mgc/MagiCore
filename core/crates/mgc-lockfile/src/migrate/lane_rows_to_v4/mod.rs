//! Wires converts one resolved lane without borrowing sibling history.
//! Nối owner: Chuyển một lane đã resolve mà không mượn lịch sử lane khác.

mod lane_rows_to_v4;
pub use self::lane_rows_to_v4::lane_rows_to_v4;
#[cfg(test)]
mod test;
