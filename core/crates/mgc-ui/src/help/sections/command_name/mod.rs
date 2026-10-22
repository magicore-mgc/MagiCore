//! Per-core command-name wiring — wiring owner tạo tên command theo core.
//! Keep the shared name rule in one private implementation.
//! Giữ quy tắc tên dùng chung tại một implementation private.

mod command_name;
pub(super) use command_name::per_core_name;

#[cfg(test)]
#[path = "test/command_name.rs"]
mod tests;
