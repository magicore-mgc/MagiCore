//! Integrity Projection owner wiring.
//! Nối owner integrity projection.

mod integrity_projection;

#[cfg(test)]
mod test;

pub(in crate::native::engine::engine) use integrity_projection::resolved_integrity;
