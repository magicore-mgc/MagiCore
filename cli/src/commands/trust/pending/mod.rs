//! Trust-pending command owner exports and wiring.
//! Wiring và export tối thiểu cho lệnh trust pending.

mod error;
mod package_scan;
mod pending;
mod policy;
mod report;
mod scan_lock;

use self::error::{PendingResult, TrustPendingError};
use self::package_scan::scan_scripted_packages;
pub use self::pending::execute;
use self::policy::{load_db_policies, load_file_policy};
use self::report::render_report;
use self::scan_lock::acquire_scan_lock;
