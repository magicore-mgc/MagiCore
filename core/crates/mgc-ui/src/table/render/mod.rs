//! Table rendering wiring — wiring render table.
//! Exposes only the parent-scope writer and registers render-line tests.
//! Chỉ expose writer trong scope cha và đăng ký test cho dòng render.

mod render;
#[cfg(test)]
#[path = "test/render.rs"]
mod tests;

pub(super) use render::print_table;
