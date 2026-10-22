//! Framework qualification catalog — catalog qualification framework.
//! Public compatibility facade; implementation is split into same-name owners.

#[path = "framework_records/mod.rs"]
mod implementation;

pub use self::implementation::{
    EV_DEPGATE, EV_E2E_APP_FLUTTER, EV_E2E_APP_SWIFT, EV_E2E_CLO_TERRAFORM, EV_E2E_HARDWARE,
    EV_E2E_IOT_PIO, EV_E2E_IOT_WEST, EV_E2E_WEB_BACKEND, EV_E2E_WEB_JS, EV_LIB_PIPELINE,
    EV_RN_GATE, EV_SCAFFOLD_WEB, EV_WEB_ENGINE, EV_WIZARD, FrameworkRecord, FrameworkStatus,
    RECORDS, qualification_json, status_string,
};

#[cfg(test)]
#[path = "test/framework_records_test.rs"]
mod tests;
