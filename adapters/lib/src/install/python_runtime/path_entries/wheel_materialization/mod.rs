//! Wires verified artifact materialization.
//! Wiring materialization artifact đã xác minh.

mod error;
mod wheel_materialization;

pub(super) use wheel_materialization::verify_python_package_materialization;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
