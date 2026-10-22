//! Bun/Deno audit result types — các kiểu kết quả audit Bun/Deno.
//! Shared data contract for both lockfile readers.
//! Hợp đồng dữ liệu dùng chung cho cả hai bộ đọc lockfile.

mod types;

pub use types::{BunDenoRead, JrSkipped};
