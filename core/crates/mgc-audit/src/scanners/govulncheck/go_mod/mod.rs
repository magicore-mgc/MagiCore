//! Go module requirement parser facade.
//! (Facade parser yêu cầu module Go.)

mod go_mod;

pub use go_mod::read_go_mod_requires;
