//! Stable public facade for Go audit and stream parsing.
//! (Facade công khai ổn định cho audit Go và parse stream.)

mod audit;
mod error;
mod finding;
mod go_mod;
#[path = "govulncheck.rs"]
mod govulncheck;
mod model;
mod parser;
mod severity;

pub use audit::audit_go;
pub use go_mod::read_go_mod_requires;
pub use govulncheck::GovulncheckParse;
pub use parser::parse_govulncheck_json;
