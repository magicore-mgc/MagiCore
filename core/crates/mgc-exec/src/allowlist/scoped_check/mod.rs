#[path = "scoped_check.rs"]
mod scoped_check;

pub use scoped_check::check_tool_scoped;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
