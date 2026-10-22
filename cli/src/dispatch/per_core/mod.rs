// Per-core routing exports — exports the dispatcher entry and typed group owners.
// Module này xuất entry dispatcher và các owner định tuyến theo nhóm.
mod add;
mod bare_common;
mod common;
mod create;
mod engine;
mod install;
mod list;
mod per_core;
mod remove;
mod update;

pub use per_core::command_to_dispatch;
