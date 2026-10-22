// Adversarial CAS integrity tests (P0-1 hardening).
// Test đối kháng cho CAS: hash sai dạng, blob bị sửa (mutation), quarantine,
// export KHÔNG là hardlink (project không ghi ngược vào store), concurrent
// writers, atomic write. Đối chiếu finding Tech Lead 2026-09-12.
#![allow(clippy::unwrap_used)] // Test code: unwrap acceptable for setup/assertions
// (Test code: unwrap chấp nhận được cho setup/assert)

#[path = "adversarial_integrity/mod.rs"]
mod adversarial_integrity;

#[test]
fn batched_cas_import_and_export_preserve_independent_verified_files() {
    adversarial_integrity::test::import_batch::batched_cas_import_and_export_preserve_independent_verified_files();
}

#[test]
fn rebuildable_batch_import_preserves_hashes_modes_and_export_validation() {
    adversarial_integrity::test::import_batch::rebuildable_batch_import_preserves_hashes_modes_and_export_validation();
}

#[test]
fn rebuildable_root_batch_export_preserves_integrity_and_independence() {
    adversarial_integrity::test::import_batch::rebuildable_root_batch_export_preserves_integrity_and_independence();
}

#[test]
fn short_hash_string_is_rejected_not_panicking() {
    adversarial_integrity::test::import_validation::short_hash_string_is_rejected_not_panicking();
}

#[test]
fn valid_hash_string_is_accepted() {
    adversarial_integrity::test::import_validation::valid_hash_string_is_accepted();
}

#[test]
fn tampered_existing_blob_fails_import_and_is_quarantined() {
    adversarial_integrity::test::quarantine::tampered_existing_blob_fails_import_and_is_quarantined(
    );
}

#[test]
fn batched_export_of_tampered_blob_is_rejected_and_quarantined() {
    adversarial_integrity::test::quarantine::batched_export_of_tampered_blob_is_rejected_and_quarantined();
}

#[cfg(unix)]
#[test]
fn batched_export_rejects_cas_symlink_even_when_target_bytes_match() {
    adversarial_integrity::test::quarantine::batched_export_rejects_cas_symlink_even_when_target_bytes_match();
}

#[test]
fn import_with_declared_hash_mismatch_is_rejected() {
    adversarial_integrity::test::import_validation::import_with_declared_hash_mismatch_is_rejected(
    );
}

#[test]
fn poisoned_import_rejected_even_when_declared_blob_exists() {
    adversarial_integrity::test::import_validation::poisoned_import_rejected_even_when_declared_blob_exists();
}

#[test]
fn import_with_malformed_declared_hash_is_rejected() {
    adversarial_integrity::test::import_validation::import_with_malformed_declared_hash_is_rejected(
    );
}

#[test]
fn exported_file_mutation_does_not_modify_store_blob() {
    adversarial_integrity::test::export_lifecycle::exported_file_mutation_does_not_modify_store_blob();
}

#[test]
fn export_twice_is_idempotent() {
    adversarial_integrity::test::export_lifecycle::export_twice_is_idempotent();
}

#[test]
fn export_missing_blob_fails_not_found() {
    adversarial_integrity::test::export_lifecycle::export_missing_blob_fails_not_found();
}

#[test]
fn tampered_tarball_dedup_entry_fails() {
    adversarial_integrity::test::quarantine::tampered_tarball_dedup_entry_fails();
}

#[test]
fn concurrent_imports_same_content_all_succeed() {
    adversarial_integrity::test::concurrency::concurrent_imports_same_content_all_succeed();
}

#[test]
fn concurrent_compiled_cache_puts_never_collide() {
    adversarial_integrity::test::concurrency::concurrent_compiled_cache_puts_never_collide();
}

#[test]
fn compiled_cache_rejects_corrupt_json_at_get() {
    adversarial_integrity::test::compiled_cache::compiled_cache_rejects_corrupt_json_at_get();
}

#[test]
fn memo_survives_nothing_same_length_rewrite_with_mtime_restore() {
    adversarial_integrity::test::verification_memo::memo_survives_nothing_same_length_rewrite_with_mtime_restore();
}

#[test]
fn memo_exec_and_nonexec_digests_do_not_share_verification() {
    adversarial_integrity::test::verification_memo::memo_exec_and_nonexec_digests_do_not_share_verification();
}

#[test]
fn memo_catches_file_replacement_by_rename() {
    adversarial_integrity::test::verification_memo::memo_catches_file_replacement_by_rename();
}

#[test]
fn staging_kind_probe_matches_platform_capability() {
    adversarial_integrity::test::filesystem_staging::staging_kind_probe_matches_platform_capability(
    );
}

#[cfg(unix)]
#[test]
fn export_to_existing_destination_repairs_executable_mode() {
    adversarial_integrity::test::export_lifecycle::export_to_existing_destination_repairs_executable_mode();
}

#[test]
fn export_destination_under_symlinked_dir_is_rejected() {
    adversarial_integrity::test::export_lifecycle::export_destination_under_symlinked_dir_is_rejected();
}

#[test]
fn typed_hash_fields_are_not_publicly_accessible() {
    adversarial_integrity::test::filesystem_guards::typed_hash_fields_are_not_publicly_accessible();
}

#[test]
fn export_temp_is_created_in_destination_filesystem() {
    adversarial_integrity::test::filesystem_staging::export_temp_is_created_in_destination_filesystem();
}

#[test]
fn export_failure_leaves_no_staging_temp() {
    adversarial_integrity::test::filesystem_staging::export_failure_leaves_no_staging_temp();
}

#[test]
fn concurrent_tarball_import_same_content_is_atomic() {
    adversarial_integrity::test::concurrency::concurrent_tarball_import_same_content_is_atomic();
}

#[test]
fn cow_or_copy_export_is_independent_from_store() {
    adversarial_integrity::test::artifact_isolation::cow_or_copy_export_is_independent_from_store();
}

#[test]
fn verified_memo_invalidates_on_disk_mutation() {
    adversarial_integrity::test::verification_memo::verified_memo_invalidates_on_disk_mutation();
}

#[test]
fn memo_catches_rewrite_with_identical_length() {
    adversarial_integrity::test::verification_memo::memo_catches_rewrite_with_identical_length();
}

#[test]
fn repeated_exports_stay_correct_with_cow() {
    adversarial_integrity::test::artifact_isolation::repeated_exports_stay_correct_with_cow();
}

#[test]
#[cfg(unix)]
fn cas_root_unreadable_metadata_fails_closed() {
    adversarial_integrity::test::filesystem_guards::cas_root_unreadable_metadata_fails_closed();
}
