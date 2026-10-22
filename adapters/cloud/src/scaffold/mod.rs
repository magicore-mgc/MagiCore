//! Cloud scaffold exports — export scaffold Cloud.

mod cdk;
mod cloudflare;
mod pulumi;
mod scaffold;
mod terraform;

pub(in crate::scaffold) use cdk::scaffold_cdk;
pub(in crate::scaffold) use cloudflare::scaffold_cloudflare;
pub(in crate::scaffold) use pulumi::scaffold_pulumi;
pub use scaffold::scaffold_project;
pub(in crate::scaffold) use terraform::scaffold_terraform;

#[cfg(test)]
#[path = "test/mod_test.rs"]
mod tests;
