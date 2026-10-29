//! Post-build work: describe, verify and sign the artefact.
//! Việc sau build: mô tả, kiểm và ký artefact.
//!
//! Building is only half of shipping. These owners run after the framework or
//! MagiCore engine has produced the output, when the files are final and can be
//! hashed, examined and signed.
//! Build chỉ là nửa đầu của việc phát hành. Các owner này chạy sau khi framework
//! hoặc engine MagiCore đã tạo output, lúc file đã cuối và có thể hash, xét và
//! ký.

#[path = "output_sbom/mod.rs"]
mod output_sbom;
#[path = "run/mod.rs"]
mod run;
#[path = "runtime_guard/mod.rs"]
mod runtime_guard;
#[path = "scan/mod.rs"]
mod scan;
#[path = "sign/mod.rs"]
mod sign;
#[path = "verify_sbom/mod.rs"]
pub(crate) mod verify_sbom;

pub(crate) use output_sbom::write as write_output_sbom;
pub(crate) use run::{PostBuildOptions, run as run_post_build, run_for_core};
pub(crate) use runtime_guard::{BuildIdentity, inject as inject_build_identity, sbom_sha256};
pub(crate) use scan::scan as scan_output;
pub(crate) use sign::{Signed, sign_artifact};
