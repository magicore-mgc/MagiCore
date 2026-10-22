//! Shared store ownership and doctor diagnostics.
//! Lỗi quyền sở hữu shared store và store doctor.

mod error;

pub use error::{
    store_core_identity_conflict, store_core_identity_conflict_any, store_core_identity_mismatch,
    store_core_marker_required, store_core_signature_required,
    store_doctor_framework_metadata_required, store_doctor_framework_mismatch,
    store_doctor_repair_unsupported, store_doctor_route_unsupported, store_doctor_unhealthy,
    store_global_root_unsafe, store_requires_web_core,
};
