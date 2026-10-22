//! Feature-gated core route wiring for shared add.
//! Wiring route core theo feature cho add dùng chung.

mod dispatch;

pub(super) use dispatch::dispatch_add;
