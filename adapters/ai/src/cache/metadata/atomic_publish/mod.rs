//! Atomic metadata writer exports — khai báo owner publish file atomic.

mod atomic_publish;

pub(super) use atomic_publish::publish_metadata;

#[cfg(test)]
#[path = "test/atomic_publish.rs"]
mod tests;
