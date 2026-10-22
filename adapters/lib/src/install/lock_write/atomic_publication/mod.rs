//! Wires atomic canonical lock publication.
//! Wiring ghi canonical lock atomic.

mod atomic_publication;

pub(in crate::install) use atomic_publication::atomic_write_canonical_lock;
