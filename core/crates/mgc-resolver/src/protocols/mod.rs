//! Phase 2 native registry protocol engines.
//! Engine registry protocol native Phase 2.
//!
//! Real HTTP engines for crates.io (Rust), PyPI (Python), pub.dev (Dart) and
//! the Go module proxy, plus the still-stubbed npm slot (the web core keeps
//! its own native path). Each engine resolves a `name` within a `range`
//! against its registry, selects the highest matching version, filters deps,
//! verifies the artifact hash and materializes the toolchain layout. The
//! engines are a LIBRARY in core — adapters wire into them, they never touch
//! adapters directly.
//! Engine HTTP thật cho crates.io (Rust), PyPI (Python), pub.dev (Dart) và
//! Go module proxy, cùng slot npm vẫn là stub (core web giữ đường native
//! riêng). Mỗi engine resolve `name` trong `range` trên registry của nó,
//! chọn version cao nhất thoả, lọc deps, xác minh hash artifact và
//! materialize layout toolchain. Engine là LIBRARY trong core — adapter wire
//! vào, engine không đụng adapter.

pub mod archive;
mod cache_segment;
#[path = "composer/mod.rs"]
pub mod composer;
pub mod crates;
pub mod go;
#[path = "hfhub/mod.rs"]
pub mod hfhub;
mod interval_range;
#[path = "maven/mod.rs"]
pub mod maven;
pub mod nuget;
#[path = "pub/mod.rs"]
pub mod pubdev;
pub mod pypi;
#[path = "reactnative/mod.rs"]
pub mod reactnative;
#[path = "swift/mod.rs"]
pub mod swift;
pub mod zip_reader;

mod digest;
mod graph_resolution;
mod npm;
mod registry_protocol;
mod registry_root;
mod resolved_entry;

pub use composer::ComposerProtocol;
pub use crates::NON_ARCHIVE_SOURCE_FILES;
pub use crates::{
    CARGO_SOURCE_MARKER, CratesProtocol, has_cargo_source_marker, registry_package_dir,
    registry_source_root, write_cargo_source_marker,
};
pub use digest::{sha256_hex, store_ref_for_blake3};
pub use go::GoModProtocol;
pub(super) use graph_resolution::GRAPH_RESOLVE_CONCURRENCY;
pub use hfhub::{HfHubProtocol, ModelFile, ModelResolution, git_blob_sha1};
pub use maven::MavenProtocol;
pub use npm::NpmProtocol;
pub use nuget::NuGetProtocol;
pub use pubdev::PubProtocol;
pub use pypi::PypiProtocol;
pub use reactnative::{
    CocoaPodsProtocol, GradleLockEntry, PODSPEC_SHA1_MARKER_PREFIX, PodfileLock,
    RN_TIER_MARKER_PREFIX,
};
pub use registry_protocol::RegistryProtocol;
pub use registry_root::RegistryRoot;
pub use resolved_entry::ResolvedEntry;
pub use swift::SwiftRegistryProtocol;
pub use swift::bump_swift_requirement;
pub use swift::remove_swift_requirement;
