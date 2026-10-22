//! Committed lifecycle-script policy and parser owner.
//! Owner policy lifecycle script đã commit và parser của nó.

#[path = "error.rs"]
mod error;
#[path = "scripts.rs"]
mod scripts;

pub use error::ScriptsPolicyError;
pub use scripts::{ScriptVerdict, ScriptsPolicy, decide_scripts, load_scripts_table, policy_lists};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
