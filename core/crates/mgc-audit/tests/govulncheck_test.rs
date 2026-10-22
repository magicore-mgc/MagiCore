//! Go scanner integration-test target facade.
//! (Facade target integration test của scanner Go.)
#![allow(clippy::unwrap_used)]
#![allow(unsafe_code)]

use mgc_audit::scanners::parse_govulncheck_json;

include!("govulncheck/fixtures.rs");
include!("govulncheck/test/stream_output.rs");
include!("govulncheck/test/stream_validation.rs");
include!("govulncheck/test/severity.rs");
include!("govulncheck/test/large_stream.rs");
include!("govulncheck/test/audit.rs");
include!("govulncheck/test/go_mod.rs");
