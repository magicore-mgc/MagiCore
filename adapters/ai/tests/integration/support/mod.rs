//! Shared integration-test support exports — export helper dùng chung.
//! Keep filesystem setup in its named owner module. — Giữ setup filesystem trong module owner cùng tên.

#[path = "support.rs"]
mod support;
pub(crate) use support::tmp;
