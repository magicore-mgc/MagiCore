//! Process-tree tracking marker and token facade.
//! Facade marker và token theo dõi cây process.

#[path = "tracking.rs"]
mod implementation;

pub(in crate::run::implementation) use implementation::{
    PROCESS_TREE_TRACKING_ENV, process_tree_tracking_token,
};
