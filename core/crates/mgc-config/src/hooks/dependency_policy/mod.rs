//! Dependency lifecycle hook policy exports.
//! Export policy hook cho lifecycle dependency.

mod dependency_policy;

#[cfg(test)]
pub(super) use dependency_policy::dependency_event_verb;
pub(super) use dependency_policy::ensure_dependency_event_hooks_allowed;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
