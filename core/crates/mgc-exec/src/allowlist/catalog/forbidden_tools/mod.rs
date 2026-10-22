#[path = "forbidden_tools.rs"]
mod forbidden_tools;

pub use forbidden_tools::FORBIDDEN_TOOLS;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
