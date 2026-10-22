//! Wire the login coordinator implementation.
//! Nối implementation của coordinator login.

#[path = "login.rs"]
mod login;

pub use login::run;
