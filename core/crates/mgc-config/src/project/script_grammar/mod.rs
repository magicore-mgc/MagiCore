//! Versioned native script grammar configuration and defaults.
//! Cấu hình grammar script native có version và giá trị mặc định.

#[path = "script_grammar.rs"]
mod script_grammar;

pub use script_grammar::{ScriptGrammarConfig, default_script_grammar};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
