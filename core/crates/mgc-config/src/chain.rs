//! Registry-chain public facade.
//! Facade công khai cho API hợp nhất registry.

#[path = "chain/mod.rs"]
mod owners;

pub use owners::*;
