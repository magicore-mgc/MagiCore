//! Provider block state transitions.
//! Các chuyển trạng thái provider block.

mod provider_state;

pub(super) use provider_state::{close_if_block_end, finish_at_eof, start_if_header};
