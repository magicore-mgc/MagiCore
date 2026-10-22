//! Wiring for previous-lock downgrade protection.
//! Wiring bảo vệ downgrade dựa trên lockfile cũ.

mod enforce_no_downgrade;

pub(super) use enforce_no_downgrade::{cached_root_resolutions, enforce_no_downgrade};
