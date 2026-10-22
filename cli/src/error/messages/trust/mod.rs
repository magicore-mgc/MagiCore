//! Trusted publishing, lock signatures, and trust policy diagnostics.
//! Lỗi trusted publishing, chữ ký lock và trust policy.

mod error;

pub use error::{
    audit_signatures_flags_conflict, lock_v4_existing_signature_invalid,
    lock_v4_legacy_signature_sidecar, trust_anchor_failed, trust_anchor_no_home,
    trust_anchor_no_marker, trust_anchor_rotate_needs_generated_key,
    trust_policy_database_open_failed, trust_policy_database_read_failed,
    trust_policy_database_write_failed, trust_policy_lock_failed, trust_policy_package_empty,
    trust_policy_path_invalid, trusted_audience_required, trusted_binding_failed,
    trusted_oci_upload_chunk_failed, trusted_oci_upload_complete_failed,
    trusted_oci_upload_location_missing, trusted_oci_upload_range_invalid,
    trusted_oci_upload_start_failed, trusted_registry_admin_required,
    trusted_registry_requires_https, trusted_registry_url_invalid, trusted_signatures_invalid,
    trusted_signatures_missing_or_mismatched, trusted_signatures_request_failed,
    trusted_signatures_response_invalid, trusted_signatures_response_too_large,
    trusted_token_exchange_failed, trusted_token_response_invalid,
};
