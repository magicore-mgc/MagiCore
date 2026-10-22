//! Export public board data and internal embedded-registry records.
//! Export board data công khai và record registry nhúng nội bộ.

mod board_types;
pub use board_types::BoardInfo;
pub(in crate::framework) use board_types::BoardRecord;
