//! Export board listing and target lookup APIs.
//! Export API liệt kê board và tra target.

mod board_queries;
pub use board_queries::{
    board_target, board_target_for_framework, boards_for_framework, known_boards,
};
