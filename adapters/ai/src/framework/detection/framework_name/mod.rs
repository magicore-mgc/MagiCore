//! AI framework-name parser module wiring.
//! Wiring module parser tên framework AI.

mod framework_name;

pub(in crate::framework) use framework_name::framework_from_str;
