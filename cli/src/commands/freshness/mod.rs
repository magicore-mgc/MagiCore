//! Cross-ecosystem freshness projections — projection freshness đa ecosystem.
//!
//! Pre-flight dependency consistency across supported ecosystems (Spec rev4
//! §8/§15 task 7): each projection extracts declared dependency names and
//! compares them with owner/ecosystem-scoped direct roots in the lock. Only
//! dependency metadata participates — script-only edits never trigger a
//! resolve.
//! (Nhất quán dependency pre-flight qua các ecosystem được hỗ trợ: mỗi
//! projection trích tên dependency khai báo và so với root trực tiếp trong
//! lock theo owner/ecosystem. Chỉ metadata dependency tham gia — sửa
//! scripts không trigger resolve.)

mod cargo;
mod flutter;
mod freshness;
mod go;
mod gradle;
mod lock;
mod manifest_file;
mod manifest_syntax;
mod maven;
mod npm;
mod nuget;
mod platformio;
mod profile;
mod python;
mod swift;
mod unity;
mod web;
mod west;

pub use cargo::{check_cargo_freshness, check_lib_cargo_freshness, declared_cargo_deps};
pub use flutter::{check_flutter_freshness, declared_flutter_deps};
pub use freshness::check_any_freshness;
pub use go::{check_web_go_build_freshness, declared_go_deps};
pub use gradle::declared_gradle_deps;
pub use maven::declared_maven_deps;
pub use npm::{check_dev_npm_freshness_against_lock, declared_npm_dep_specs};
pub use nuget::declared_nuget_deps;
pub use platformio::declared_platformio_deps;
pub use profile::{check_profile_gate, default_profile_store_dir};
pub use python::{check_python_freshness, declared_python_deps};
pub use swift::{check_swift_freshness, declared_swift_deps};
pub use unity::declared_unity_deps;
pub use web::resolve_web_framework;
pub use west::declared_west_deps;
