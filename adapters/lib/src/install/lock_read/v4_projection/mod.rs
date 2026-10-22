//! Wires V4 lock graph projection.
//! Wiring chiếu graph lock V4.

mod v4_projection;

pub(in crate::install) use v4_projection::project_v4_lock_to_lib_model;
