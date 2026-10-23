//! Compatibility exporters: canonical mgc.lock → ecosystem lockfile shapes facade.
//! Facade exporter tương thích: mgc.lock canonical → shape lockfile hệ sinh thái.
//!
//! Phase 1 scope: SKELETON output — structurally correct conduits so legacy
//! toolchains can consume the canonical graph. They are NOT byte-perfect
//! toolchain lockfiles, never carry full toolchain metadata (checksums,
//! hashes), and never feed back into mgc.lock — mgc.lock stays the single
//! source of truth.
//! Phạm vi Phase 1: output SKELETON — đúng cấu trúc để toolchain cũ tiêu
//! thụ đồ thị canonical. KHÔNG phải lockfile byte-perfect của toolchain,
//! không mang đủ metadata của toolchain (checksum, hash) và không bao giờ
//! đổ ngược vào mgc.lock — mgc.lock vẫn là nguồn truth duy nhất.

mod cargo_lock;
mod dispatch;
mod go_sum;
mod package_lock;
mod requirements_txt;
mod shared;
mod target;

pub use dispatch::export_canonical;
pub use target::ExportTarget;
