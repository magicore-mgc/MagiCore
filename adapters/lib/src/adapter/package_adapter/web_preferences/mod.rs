//! Web-specific preferences forwarded through the Lib facade.
//! Các tùy chọn Web được chuyển tiếp qua facade Lib.

mod web_preferences;

pub(super) use web_preferences::{set_dedupe_pref, set_existing_versions};
