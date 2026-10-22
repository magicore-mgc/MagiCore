//! Writes sorted peer contexts and their stable key/value encoding.
//! File này chỉ nối owner peer_context_section.

mod peer_context_section;

pub(crate) use peer_context_section::append_peer_context_section;
