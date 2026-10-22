//! Framework record owner wiring — nối các owner record framework.
#[path = "dependency/mod.rs"]
mod dependency;
#[path = "framework_records.rs"]
mod implementation;
#[path = "records/mod.rs"]
mod records;
#[path = "types/mod.rs"]
mod types;

pub use self::implementation::qualification_json;
pub use self::implementation::status_string;
pub use self::records::RECORDS;
#[allow(unused_imports)]
// Public record types are shared by nested records and compatibility callers.
pub use self::types::{
    EV_DEPGATE, EV_E2E_APP_FLUTTER, EV_E2E_APP_SWIFT, EV_E2E_CLO_TERRAFORM, EV_E2E_HARDWARE,
    EV_E2E_IOT_PIO, EV_E2E_IOT_WEST, EV_E2E_WEB_BACKEND, EV_E2E_WEB_JS, EV_LIB_PIPELINE,
    EV_RN_GATE, EV_SCAFFOLD_WEB, EV_WEB_ENGINE, EV_WIZARD, FrameworkRecord, FrameworkStatus,
};
