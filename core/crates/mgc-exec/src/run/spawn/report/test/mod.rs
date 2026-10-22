//! Register report and exit-policy behavior cases.
//! Đăng ký ca hành vi report và chính sách exit.

use super::{build_exec_report, ensure_exit_code_allowed};

include!("bounded_report/mod.rs");
include!("exit_code_policy/mod.rs");
