//! Compatibility facade for child-process wait supervision.
//! Facade tương thích cho giám sát chờ process con.

#[path = "error.rs"]
mod error;
#[path = "wait_session/mod.rs"]
mod wait_session;

pub(in crate::run::implementation) use wait_session::wait_with_timeout;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
