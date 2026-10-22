//! Resolve a recognized Game framework into a native dev command.
//! Phân giải Game framework được nhận diện thành native dev command.

mod framework_command;

pub(super) use framework_command::game_dev_command;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
