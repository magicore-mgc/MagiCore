//! Stable facade for the native Go module proxy protocol.
//! Facade ổn định cho protocol Go module proxy native.

mod cache;
mod go;
mod http;
mod integrity;
mod manifest;
mod metadata;
mod resolve;
mod version;

pub use self::go::{GoModProtocol, GoModuleFiles};
pub use self::integrity::dirhash_zip;
pub use self::manifest::{GoModFile, GoReplace, GoRequire, parse_go_mod};

pub(in crate::protocols::go) use self::cache::escape_module_path;
pub(in crate::protocols::go) use self::go::GO_MOD_SHA256_MARKER_PREFIX;
pub(in crate::protocols::go) use self::metadata::{GoInfo, GoLatest, validate_go_info};
pub(in crate::protocols::go) use self::version::{go_matches, parse_go_version};
