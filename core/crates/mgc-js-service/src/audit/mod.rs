//! Shared npm Bulk Advisory pipeline primitives (S07 seed).
//!
//! Owns the registry-trust predicate, advisory endpoint builder, bulk request
//! shaper, and fail-closed response parser. The Web facade continues to expose
//! the existing caller paths.
//! Sở hữu predicate trust registry, builder endpoint advisory, shaper request
//! bulk và parser response fail-closed. Facade Web tiếp tục giữ các đường gọi
//! hiện có.
//!
//! The Web-owned run_audit runner, run_audit_fix, lockfile pin sourcing,
//! legacy lockfile reader, and rival-lockfile gate stay in the adapter until
//! their lockfile seam is migrated; moving them here would duplicate lock
//! ownership or create an adapter/service cycle.
//! Runner run_audit, run_audit_fix, nguồn pin lockfile, reader lockfile cũ và
//! gate rival-lockfile vẫn do Web sở hữu tới khi seam lockfile được migrate;
//! chuyển chúng vào đây sẽ trùng ownership hoặc tạo vòng adapter/service.

mod request;
mod response;
mod trust;

pub use request::{advisory_bulk_request_body, registry_advisory_bulk_endpoint};
pub use response::parse_advisory_bulk_response;
pub use trust::is_tarball_url_trusted;
