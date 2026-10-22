#[path = "simple_script.rs"]
mod simple_script;

pub use simple_script::parse_simple_script;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
