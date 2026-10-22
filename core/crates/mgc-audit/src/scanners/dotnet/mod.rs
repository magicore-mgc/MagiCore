//! Wire .NET NuGet audit owners and preserve the scanner API.
//! Nối các owner audit NuGet/.NET và giữ API scanner ổn định.

mod dotnet;
mod error;
mod lockfile;
mod pin_collection;
mod project_path;
mod solution;

pub use self::dotnet::audit_dotnet;
pub use self::lockfile::read_packages_lock;
pub use self::pin_collection::collect_dotnet_pins;
pub use self::solution::read_solution_projects;
