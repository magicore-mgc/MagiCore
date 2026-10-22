//! Export bearer identity resolution.
//! Xuất chức năng phân giải danh tính bearer.

mod identity;
pub(in crate::npm) use identity::whoami;
