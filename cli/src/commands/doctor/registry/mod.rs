//! Registry reachability collection.
//! Thu thập trạng thái reachability registry.

mod registry;

#[cfg(test)]
pub(super) use registry::build_statuses;
pub(super) use registry::statuses;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
