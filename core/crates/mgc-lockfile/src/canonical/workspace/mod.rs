//! Converts the accepted v4 workspace and cross-edge wire shape into shared schema types.
//! File này chỉ nối và export owner workspace.

mod workspace;

pub(super) use workspace::deserialize_v4_workspace;
