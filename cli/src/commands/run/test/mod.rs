//! Run-command test owner wiring and shared fixture.
//! Wiring test lệnh run và fixture dùng chung.

use super::{resolve_mgc_toml_script, resolve_package_json_script};

mod execution_policy;
mod fixture_writer;
mod lifecycle;
mod script_resolution;
use fixture_writer::write;
