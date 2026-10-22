#[path = "script_chain.rs"]
mod script_chain;

pub use script_chain::parse_script_chain;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
