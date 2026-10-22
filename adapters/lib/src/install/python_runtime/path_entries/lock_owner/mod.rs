//! Wires Python lock owner loading.
//! Wiring đọc lock thuộc Python.

mod error;
mod lock_owner;

pub(super) use lock_owner::read_python_lock;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
