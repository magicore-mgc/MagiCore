//! Shared integration-test support exports — export helper test dùng chung.
//! Filesystem setup stays in the named helper owner. — Setup filesystem nằm ở helper owner cùng tên.

#[path = "support.rs"]
mod support;
pub(crate) use support::tmp;
