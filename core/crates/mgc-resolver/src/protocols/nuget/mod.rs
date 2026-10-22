//! Stable facade for the native NuGet v3 protocol.
//! Facade ổn định cho protocol NuGet v3 native.

mod cache;
mod framework;
mod http;
mod manifest;
mod nuget;
mod registration;
mod resolve;
mod version;
mod xml;

pub use self::manifest::{PackageReference, parse_package_references};
pub use self::nuget::NuGetProtocol;

pub(in crate::protocols::nuget) use self::framework::{group_tfm, tfm_compat_rank};
pub(in crate::protocols::nuget) use self::registration::find_resource;
pub(in crate::protocols::nuget) use self::version::nuget_matches;
pub(in crate::protocols::nuget) use self::xml::{tag_attr, tag_blocks};
