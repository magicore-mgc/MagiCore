mod git_checks;
mod types;
pub(in crate::commands) use git_checks::git_checks;
#[cfg(test)]
pub(in crate::commands) use git_checks::{git_exec_options, parse_git_behind_count};
pub(in crate::commands) use types::GitReadQuery;
