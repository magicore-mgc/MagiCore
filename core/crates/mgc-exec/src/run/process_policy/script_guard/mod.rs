//! Script-guard owner exports.
//! Export owner guard script.

#[path = "script_guard.rs"]
mod implementation;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(in crate::run::implementation) use implementation::reject_forbidden_script_file;
