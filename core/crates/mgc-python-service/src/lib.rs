//! mgc-python-service — shared native Python (PEP 621) service seed.
//! Shared native Python (PEP 621) service seed — hạt giống service Python
//! native dùng chung.
//!
//! Responsibility / Trách nhiệm: own the MGC-native Python lane's pure
//! domain logic — PEP 621 ownership predicate, `pyproject.toml` manifest IO,
//! core-owner selection, and native-install leaf helpers (artifact-cache
//! read, importability gate). The generic resolve→lock wiring
//! (`native::engine`) and the store/CAS-backed install orchestration stay
//! with their current owners (see module docs).
//!
//! Owner / Chủ sở hữu: shared service crate (S09a, D012 PROPOSED). Consumers:
//! `mgc-lib-adapter` (via one-line COMPAT facades, S09b-cleanup pending);
//! `mgc-ai-adapter` retarget is S09b (NOT this batch — ai still goes through
//! the lib facades).
//!
//! Callers / Nơi gọi: `adapters/lib/src/manifest.rs` facades (6 fns),
//! `adapters/lib/src/install/mod.rs` facades (8 fns); indirect: `ai`
//! (`supports_native_python_project` via lib re-export), `cli`
//! (`native_python_path_entries`, body stays in lib).
//!
//! Excluded (deliberately NOT moved) / Không chuyển (chủ ý giữ lại):
//! - `native::engine::{resolve_with_protocol, resolve_with_protocol_including_dev}`
//!   — generic across Rust/Go/Java/DotNet (+ app Flutter/Swift callers); moves
//!   in the P04 wave, not for Python alone (S06 D013).
//! - `install_python_native` orchestrator + `native_python_path_entries{,_from_store}`
//!   + `native_python_installed_packages` — depend on lib-owned install infra
//!     (`shared_store::SharedStoreRun`, `content_store`, `read_existing_lock`);
//!     they call into this crate's leaf helpers through same-name facades.
//! - `read_existing_lock` family — generic lock reading (Rust callers in
//!   `cargo_compat.rs`); wrong owner for a Python-named crate.
//!
//! Error surface / Bề mặt lỗi: no new failure contracts — every fallible
//! function returns `mgc_types::MgResult`/`mgc_types::MgError` passthrough.
//! Per A08 there is no `error.rs` here (nothing new to classify); failures
//! stay typed at the `mgc-types` owner. (Không có error enum mới; mọi lỗi
//! chuyển tiếp `MgError` từ `mgc-types`.)

// Test fixtures use unwrap for concise setup failures; production code keeps the workspace lint.
#![cfg_attr(test, allow(clippy::unwrap_used))]

pub mod install;
pub mod manifest;
