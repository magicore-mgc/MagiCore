//! Native Web development-server owner wiring.
//! Wiring owner dev-server Web native.
#[path = "compile_options/mod.rs"]
mod compile_options;
#[path = "compiled_cache/mod.rs"]
mod compiled_cache;
#[path = "compiler/mod.rs"]
mod compiler;
#[path = "dependency_route/mod.rs"]
mod dependency_route;
#[path = "html_route/mod.rs"]
mod html_route;
#[path = "imports/mod.rs"]
mod imports;
#[path = "request_path/mod.rs"]
mod request_path;
#[path = "server/mod.rs"]
mod server;
#[path = "source_file_resolution/mod.rs"]
mod source_file_resolution;
#[path = "source_route/mod.rs"]
mod source_route;
#[path = "transpile/mod.rs"]
mod transpile;
#[path = "tsconfig/mod.rs"]
mod tsconfig;
#[path = "types/mod.rs"]
mod types;

pub use self::server::MgDevServer;
pub use self::types::DevServerConfig;
