//! Process-policy coordinator — keep matcher order and stable exports here.
//! Điều phối process-policy — giữ thứ tự matcher và export ổn định tại đây.

use super::super::{
    FORBIDDEN_TOOLS, dependency_resolution_deny_reason, forbidden_entry_name,
    parse_runtime_invocation, process_invocation_program,
};
#[cfg(any(windows, test))]
use super::ProcessPolicyError;
use super::process_basename;

#[path = "argv/mod.rs"]
mod argv;
#[cfg(any(windows, test))]
#[path = "child_command_line/mod.rs"]
mod child_command_line;
#[path = "command_line/mod.rs"]
mod command_line;

#[cfg(test)]
pub(in crate::run::implementation) use self::argv::forbidden_process_match_argv_with_env;
pub(in crate::run::implementation) use self::argv::{
    forbidden_process_match_argv, forbidden_process_match_argv_with_scope,
};
#[cfg(any(windows, test))]
pub(in crate::run::implementation) use self::child_command_line::validate_monitored_child_command_line;
#[cfg(test)]
pub(in crate::run::implementation) use self::command_line::forbidden_process_match;
pub(in crate::run::implementation) use self::command_line::forbidden_process_name;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
