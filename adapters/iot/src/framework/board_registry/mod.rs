//! Export access to the validated embedded board registry.
//! Export truy cập registry board nhúng đã validate.

mod board_registry;
pub(in crate::framework) use board_registry::board_records;
