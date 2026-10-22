//! Node argv parser exports.
//! Export parser argv Node.

#[path = "node.rs"]
mod implementation;
pub(super) use implementation::node_script_argument;
