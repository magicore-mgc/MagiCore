//! Shared test support wiring — wiring hỗ trợ test dùng chung.
//! Exposes test-only constructors without adding production dependencies.
//! Chỉ xuất fixture trong test, không thêm dependency production.

mod test_support;
pub(crate) use crate::commands::dep_gate::DepContext;
pub(crate) use crate::commands::dep_gate::DepOp;
pub(crate) use crate::commands::dep_gate::DepOwner;
pub(crate) use crate::commands::dep_gate::eco;
pub(crate) use crate::commands::dep_gate::from_dep_flag;
pub(crate) use crate::commands::dep_gate::gate;
pub(crate) use crate::commands::dep_gate::owner_for;
pub(crate) use test_support::{ctx, explicit, lib_ctx, native};
