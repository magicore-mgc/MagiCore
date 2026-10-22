//! Native Cloud deploy API exports — export API deploy native Cloud.

mod deploy;
mod result;

pub use deploy::deploy;
pub use result::DeployResult;

#[cfg(test)]
#[path = "test/mod_test.rs"]
mod tests;
