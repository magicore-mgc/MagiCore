//! Feature-gated core route wiring for shared remove.
//! Wiring route core theo feature cho remove dùng chung.

mod dispatch;

pub(super) use dispatch::dispatch_remove;
