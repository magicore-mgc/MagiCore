//! Operation projection wiring — nối projection operation.
//! The leaf owns one operation row for a gate context.
//! File lá sở hữu một hàng operation theo context gate.

mod operation_projection;

pub(super) use operation_projection::operations_for;
