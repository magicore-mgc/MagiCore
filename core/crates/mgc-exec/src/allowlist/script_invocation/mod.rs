#[path = "script_invocation.rs"]
mod script_invocation;

pub use script_invocation::parse_script_invocation;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
