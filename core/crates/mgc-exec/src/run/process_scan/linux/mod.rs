//! Linux process scan facade — export Linux process snapshots.
//! Facade quét process Linux — export snapshot process Linux.

#[path = "linux.rs"]
mod implementation;
// The item is `pub(in crate::run::implementation)`, so the hop that forwards it
// must not narrow it to `pub(super)` — that is the E0364 Linux build error.
// (Item là `pub(in crate::run::implementation)`, nên hop chuyển tiếp không được
// thu hẹp về `pub(super)` — đó là lỗi E0364 khi build Linux.)
pub(in crate::run::implementation) use implementation::{
    linux_process_snapshots, linux_process_stat_snapshot,
};
