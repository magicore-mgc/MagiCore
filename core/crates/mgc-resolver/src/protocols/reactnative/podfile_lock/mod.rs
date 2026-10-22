//! Podfile.lock owner wiring.
//! Nối các owner xử lý Podfile.lock.

#[path = "podfile_lock.rs"]
mod implementation;
#[path = "parse/mod.rs"]
mod parse;

pub use implementation::{PodfileLock, PodfileLockPod, pod_root};
pub use parse::parse_podfile_lock;
