//! Test support wiring.
//! Wiring hỗ trợ test.

mod mock_core;
pub(super) use mock_core::MockCore;
pub(super) use mock_core::assert_unsupported;
