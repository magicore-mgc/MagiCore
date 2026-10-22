//! Mutation journal parser wiring.
//! Wiring parser journal mutation.

#[path = "error.rs"]
mod error;
#[path = "parser.rs"]
mod implementation;

pub(super) use implementation::parse_mutation_journal;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
