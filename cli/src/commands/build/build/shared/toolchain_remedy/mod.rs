//! Toolchain remedy owner — wiring only.
//! Owner khắc phục toolchain — chỉ wiring.

#[path = "toolchain_remedy.rs"]
mod toolchain_remedy;

#[cfg(test)]
pub(crate) use toolchain_remedy::toolchain_remedy;
/// Classification is exercised through its own absent tests; only the context
/// and the appender are needed by build lanes.
/// (Phân loại được test riêng; lane build chỉ cần context và hàm nối câu.)
pub(crate) use toolchain_remedy::{ToolchainContext, with_remedy};
