#[path = "reject_forbidden_pm_script.rs"]
mod reject_forbidden_pm_script;

pub use reject_forbidden_pm_script::reject_forbidden_pm_script;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
