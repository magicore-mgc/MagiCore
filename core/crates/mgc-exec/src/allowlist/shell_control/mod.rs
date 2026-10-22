#[path = "shell_control.rs"]
mod shell_control;

pub use shell_control::reject_shell_control;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
