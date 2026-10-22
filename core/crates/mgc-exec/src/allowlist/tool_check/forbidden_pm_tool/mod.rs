#[path = "forbidden_pm_tool.rs"]
mod forbidden_pm_tool;

pub(super) use forbidden_pm_tool::reject_forbidden_pm_tool;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
