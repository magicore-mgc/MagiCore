//! `adapters/mod.rs` — Optimizer Adapter Pattern facade (runtime-specific optimizations).
//! Facade mẫu Adapter cho Optimizer (tối ưu hóa theo từng runtime).

pub mod bun;
pub mod candle;
pub mod deno;
pub mod flutter;
pub mod go_ai;
pub mod go_lib;
pub mod node;
pub mod python_lib;
pub mod pytorch;
pub mod react_native;
pub mod rust_lib;
pub mod typescript_lib;

mod contract;
mod registry;
mod selection;

pub use contract::OptimizerAdapter;
pub use registry::all_adapters;
pub use selection::find_adapters;
