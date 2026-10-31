//! Export board listing, target and chip lookup APIs.
//! Export API liệt kê board, tra target và chip.

mod board_queries;
pub use board_queries::{
    board_chip_for_framework, board_target, board_target_for_framework, boards_for_framework,
    known_boards,
};
