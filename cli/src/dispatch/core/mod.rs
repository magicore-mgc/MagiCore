//! Per-core dispatch facade.
//! Facade dispatch theo core.

mod create;
mod dispatch;
mod install;
mod package_ops;

pub use dispatch::dispatch_core;
