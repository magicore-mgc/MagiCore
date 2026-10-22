//! Dependency Closure owner wiring.
//! Nối owner dependency closure.

mod dependency_closure;

#[cfg(test)]
mod test;

pub(in crate::native::engine::engine) use dependency_closure::runtime_reachable;
