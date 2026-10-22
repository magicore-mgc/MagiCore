#[path = "non_empty_tool_name.rs"]
mod non_empty_tool_name;

pub(super) use non_empty_tool_name::validate_non_empty_tool_name;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
