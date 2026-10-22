//! Table API wiring — wiring API table.
//! The existing public Table type remains available at the same module path.
//! Giữ type Table public tại module path hiện hành.

mod render;
mod table;

pub use table::Table;
