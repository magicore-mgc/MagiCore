//! Shared integration-test support exports — export helper test dùng chung.
//! Filesystem setup stays in its named helper owner. — Setup filesystem nằm trong helper owner cùng tên.

#[path = "support.rs"]
mod support;
pub(crate) use support::tmp;
