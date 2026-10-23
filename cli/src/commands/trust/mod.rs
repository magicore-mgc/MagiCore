//! Trust commands for lockfile signing and verification
//! Lệnh trust cho ký và xác minh lockfile

pub mod anchor;
pub mod approve;
pub mod deny;
pub mod init;
pub mod list;
pub mod pending;
pub mod policy;
pub mod prune;
pub mod script_policy;
pub mod sign;
pub mod verify;

mod run;
mod schema;

pub use run::{execute, run};
pub use schema::TrustCmd;
