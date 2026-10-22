//! Mutation rollback module wiring.
//! Wiring module rollback mutation.

#[path = "rollback.rs"]
mod implementation;

#[cfg(test)]
pub(crate) use implementation::combine_rollback_errors;
pub(crate) use implementation::rollback_mutation;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
