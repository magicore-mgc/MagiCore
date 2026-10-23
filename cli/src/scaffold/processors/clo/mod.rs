//! Cloud scaffold processor owner.
//! Owner processor scaffold cloud.

mod cdk;
mod clo;
mod cloudflare;
mod common;
mod lambda;
mod pulumi;
mod selection;
mod terraform;

pub use clo::CloProcessor;
