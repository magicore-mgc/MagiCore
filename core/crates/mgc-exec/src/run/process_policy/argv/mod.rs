//! OS argv process matcher exports.
//! Export matcher process từ OS argv.

#[path = "argv.rs"]
mod implementation;
#[cfg(test)]
pub(in crate::run::implementation) use implementation::forbidden_process_match_argv_with_env;
pub(in crate::run::implementation) use implementation::{
    forbidden_process_match_argv, forbidden_process_match_argv_with_scope,
};
