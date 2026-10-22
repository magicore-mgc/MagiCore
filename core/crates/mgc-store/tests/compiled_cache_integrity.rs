// Compiled-cache adversarial tests (P0-1/P0-2/P0-3 audit vòng-4).
// Test đối kháng cho compiled-cache: key đầy đủ ngữ cảnh, record tự xác
// thực, race nhất quán giữa các OS, LRU memo có giới hạn. Đối chiếu
// checklist Tech Lead 2026-09-13.
#![allow(clippy::unwrap_used)]
// Test code: unwrap acceptable for setup/assertions
// (Test code: unwrap chấp nhận được cho setup/assert)

//! Stable Cargo integration-test target; behavior lives in compiled_cache_integrity/test owners.
//! Cargo target integration ổn định; hành vi nằm trong các owner compiled_cache_integrity/test.

#[path = "compiled_cache_integrity/mod.rs"]
mod compiled_cache_integrity;

#[test]
fn same_source_different_loaders_get_distinct_entries() {
    compiled_cache_integrity::test::key_context::same_source_different_loaders_get_distinct_entries(
    );
}

#[test]
fn same_source_different_compiler_versions_get_distinct_entries() {
    compiled_cache_integrity::test::key_context::same_source_different_compiler_versions_get_distinct_entries();
}

#[test]
fn same_source_different_options_get_distinct_entries() {
    compiled_cache_integrity::test::key_context::same_source_different_options_get_distinct_entries(
    );
}

#[test]
fn malformed_key_inputs_are_rejected() {
    compiled_cache_integrity::test::key_context::malformed_key_inputs_are_rejected();
}

#[test]
fn poisoned_valid_json_is_rejected_and_quarantined() {
    compiled_cache_integrity::test::record_validation::poisoned_valid_json_is_rejected_and_quarantined();
}

#[test]
fn valid_json_with_wrong_embedded_key_is_rejected() {
    compiled_cache_integrity::test::record_validation::valid_json_with_wrong_embedded_key_is_rejected();
}

#[test]
fn stale_schema_entry_reports_miss_and_is_replaced() {
    compiled_cache_integrity::test::record_validation::stale_schema_entry_reports_miss_and_is_replaced();
}

#[test]
fn truncated_entry_fails_at_get_and_quarantines() {
    compiled_cache_integrity::test::record_validation::truncated_entry_fails_at_get_and_quarantines(
    );
}

#[test]
fn concurrent_same_key_same_payload_all_succeed() {
    compiled_cache_integrity::test::concurrency::concurrent_same_key_same_payload_all_succeed();
}

#[test]
fn sequential_same_key_divergent_payload_conflicts() {
    compiled_cache_integrity::test::concurrency::sequential_same_key_divergent_payload_conflicts();
}

#[test]
fn concurrent_same_key_divergent_payloads_deterministic_outcome() {
    compiled_cache_integrity::test::concurrency::concurrent_same_key_divergent_payloads_deterministic_outcome();
}

#[test]
#[cfg(unix)]
fn memo_is_bounded_and_evicts_lru() {
    compiled_cache_integrity::test::verification_memo::memo_is_bounded_and_evicts_lru();
}

#[test]
#[cfg(unix)]
fn memo_replacement_is_not_counted_as_eviction() {
    compiled_cache_integrity::test::verification_memo::memo_replacement_is_not_counted_as_eviction(
    );
}

#[test]
#[cfg(unix)]
fn memo_full_cache_replacement_is_not_counted_as_eviction() {
    compiled_cache_integrity::test::verification_memo::memo_full_cache_replacement_is_not_counted_as_eviction();
}

#[test]
#[cfg(unix)]
fn memo_hits_count_and_mutation_still_fails() {
    compiled_cache_integrity::test::verification_memo::memo_hits_count_and_mutation_still_fails();
}

#[test]
#[cfg(unix)]
fn cas_root_that_is_a_symlink_is_rejected() {
    compiled_cache_integrity::test::root_security::cas_root_that_is_a_symlink_is_rejected();
}

#[test]
#[cfg(unix)]
fn cas_root_under_symlinked_ancestor_is_rejected() {
    compiled_cache_integrity::test::root_security::cas_root_under_symlinked_ancestor_is_rejected();
}

#[test]
#[cfg(unix)]
fn cas_root_dangling_symlink_is_rejected() {
    compiled_cache_integrity::test::root_security::cas_root_dangling_symlink_is_rejected();
}

#[test]
fn cas_root_regular_dir_is_accepted() {
    compiled_cache_integrity::test::root_security::cas_root_regular_dir_is_accepted();
}

#[test]
#[cfg(unix)]
fn cas_root_and_blobs_are_owner_only() {
    compiled_cache_integrity::test::root_security::cas_root_and_blobs_are_owner_only();
}

#[test]
#[cfg(unix)]
fn system_root_owned_symlink_ancestor_is_tolerated() {
    compiled_cache_integrity::test::root_security::system_root_owned_symlink_ancestor_is_tolerated(
    );
}

#[test]
fn output_digest_none_is_not_some_empty() {
    compiled_cache_integrity::test::digest_framing::output_digest_none_is_not_some_empty();
}

#[test]
fn output_digest_nul_in_payload_is_not_ambiguous() {
    compiled_cache_integrity::test::digest_framing::output_digest_nul_in_payload_is_not_ambiguous();
}

#[test]
fn output_digest_same_module_same_digest() {
    compiled_cache_integrity::test::digest_framing::output_digest_same_module_same_digest();
}

#[test]
fn attacker_recompute_is_served_documented_boundary() {
    compiled_cache_integrity::test::threat_boundary::attacker_recompute_is_served_documented_boundary();
}

#[test]
fn record_with_garbage_loader_json_fails_to_parse() {
    compiled_cache_integrity::test::record_validation::record_with_garbage_loader_json_fails_to_parse();
}

#[test]
fn record_with_junk_digest_key_fails_to_parse() {
    compiled_cache_integrity::test::record_validation::record_with_junk_digest_key_fails_to_parse();
}

#[test]
fn staging_claims_survive_other_promote_and_abort_is_scoped() {
    compiled_cache_integrity::test::generation_claims::
        staging_claims_survive_other_promote_and_abort_is_scoped();
}
