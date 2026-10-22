//! Resolve a recognized IoT framework into its current dev command.
//! Phân giải IoT framework được nhận diện thành dev command hiện tại.

mod framework_command;

pub(super) use framework_command::iot_dev_command;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
