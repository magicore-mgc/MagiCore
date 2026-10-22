#![allow(clippy::unwrap_used)]
//! Adversarial regression for the generation-TOKEN protocol (Gate 11-A,
//! vòng-11 audit): the round-10 schema fixed the PK, but the token
//! lifecycle was unguarded — promote/abort/claim ran mutations BEFORE
//! verifying the token, allocation was a two-statement race, and nothing
//! enforced "staging-only mutation". These tests pin each fix.
//! test riêng tại test/ (RULE §5).
//! (Test hồi quy đối kháng cho giao thức generation-TOKEN: schema vòng-10
//! sửa PK, nhưng vòng đời token chưa có bảo vệ — promote/abort/claim
//! mutate TRƯỚC khi verify token, cấp phát là race 2-statement, không gì
//! ép "chỉ staging được mutate". Các test này ghim từng bản sửa.)

#[path = "generation_token/mod.rs"]
mod generation_token;

// ── P0-2: promote with a forged/unknown token must be zero-mutation ──

#[test]
fn promote_unknown_token_is_zero_mutation() {
    generation_token::test::token_transitions::promote_unknown_token_is_zero_mutation();
}

#[test]
fn promote_promoted_token_is_verified_noop() {
    generation_token::test::token_transitions::promote_promoted_token_is_verified_noop();
}

// ── P0-3: abort must never touch a promoted generation ────────────────

#[test]
fn abort_promoted_token_is_hard_error_claims_intact() {
    generation_token::test::token_transitions::abort_promoted_token_is_hard_error_claims_intact();
}

#[test]
fn cas_claim_batch_is_atomic_and_rejects_non_staging_tokens() {
    generation_token::test::claim_admission::cas_claim_batch_is_atomic_and_rejects_non_staging_tokens();
}

#[test]
fn cas_claim_batch_reports_failing_insert_and_rolls_back_prior_claims() {
    generation_token::test::claim_admission::cas_claim_batch_reports_failing_insert_and_rolls_back_prior_claims();
}

#[test]
fn concurrent_claim_batches_preserve_every_package_file_claim() {
    generation_token::test::allocation_races::concurrent_claim_batches_preserve_every_package_file_claim();
}

#[test]
fn abort_unknown_token_is_idempotent_noop() {
    generation_token::test::token_transitions::abort_unknown_token_is_idempotent_noop();
}

// ── P0-4: claims reject forged / cross-project / promoted tokens ───────

#[test]
fn claim_rejects_forged_token() {
    generation_token::test::claim_admission::claim_rejects_forged_token();
}

#[test]
fn claim_rejects_cross_project_token() {
    generation_token::test::claim_admission::claim_rejects_cross_project_token();
}

#[test]
fn claim_rejects_promoted_token() {
    generation_token::test::claim_admission::claim_rejects_promoted_token();
}

// ── Aborted-token API rejection and fresh-schema FK enforcement (P0-4) ─

#[test]
fn aborted_generation_rejects_claim_via_api() {
    generation_token::test::claim_admission::aborted_generation_rejects_claim_via_api();
}

#[test]
fn foreign_key_rejects_orphan_claim_in_fresh_schema() {
    generation_token::test::claim_admission::foreign_key_rejects_orphan_claim_in_fresh_schema();
}

#[test]
fn cas_integrity_snapshot_counts_corrupt_rows() {
    generation_token::test::integrity_snapshot::cas_integrity_snapshot_counts_corrupt_rows();
}

// ── P0-1: atomic allocation under a REAL thread barrier ────────────────

#[test]
fn begin_cas_generation_survives_barrier_race() {
    generation_token::test::allocation_races::begin_cas_generation_survives_barrier_race();
}

// ── RAII guard (Gate 11-A item 6) ─────────────────────────────────────

#[test]
fn generation_guard_aborts_staging_on_drop() {
    generation_token::test::install_coordination::generation_guard_aborts_staging_on_drop();
}

#[test]
fn generation_guard_disarm_keeps_promoted_claims() {
    generation_token::test::install_coordination::generation_guard_disarm_keeps_promoted_claims();
}

// ── Project install lock (Gate 11-A item 7) ────────────────────────────

#[test]
fn project_install_lock_serializes_same_project() {
    generation_token::test::install_coordination::project_install_lock_serializes_same_project();
}

// ── Crash semantics preserved from v2 (regression) ─────────────────────

#[test]
fn crashed_staging_claims_survive_other_promote_v3() {
    generation_token::test::token_transitions::crashed_staging_claims_survive_other_promote_v3();
}

// ── Migration with the FK rebuild (v2 → v3) keeps claims ───────────────

#[test]
fn migration_v2_shape_preserves_claims_across_api_promote() {
    generation_token::test::migration::migration_v2_shape_preserves_claims_across_api_promote();
}

// ── Crash-recovery lease (Gate 11-B, vòng-11) ──────────────────────────

#[test]
fn staging_leases_report_pid_and_age() {
    generation_token::test::staging_leases::staging_leases_report_pid_and_age();
}

#[test]
fn legacy_staging_without_lease_still_lists() {
    generation_token::test::staging_leases::legacy_staging_without_lease_still_lists();
}

// ── Claim-map GC query (P2-2, fresh-context review 2026-09-15) ────────

/// The covered-by-promoted query names exactly the claim-ful staging
/// generations whose EVERY claimed hash is still claimed by a PROMOTED
/// generation of the same project — retiring them deletes no live-blob
/// protection. A staging with even ONE uncovered hash must NOT be named.
/// (Truy vấn covered-by-promoted gọi tên đúng các staging CÓ claim mà
/// MỌI hash được PROMOTED generation của cùng project giữ — nghỉ hưu
/// chúng không xóa bảo vệ blob sống nào. Staging có MỘT hash chưa được
/// giữ thì KHÔNG được gọi tên.)
#[test]
fn covered_by_promoted_names_only_safe_claimful_staging() {
    generation_token::test::promoted_coverage::covered_by_promoted_names_only_safe_claimful_staging(
    );
}

/// Cross-project isolation: a staging of project A whose claims are
/// covered only by PROMOTED rows of project B must never be named —
/// coverage is per-project (B's claims protect nothing for A).
/// (Cô lập chéo project: staging của project A mà claim chỉ được row
/// PROMOTED của project B giữ thì không bao giờ được gọi tên — độ phủ
/// tính theo từng project (claim của B không bảo vệ gì cho A).)
#[test]
fn covered_by_promoted_is_project_scoped() {
    generation_token::test::promoted_coverage::covered_by_promoted_is_project_scoped();
}

/// Live-lease staging (the common case DURING an install) is never named
/// — the query is evidence-only; the doctor's pid/lock gates decide.
/// Pin the row SHAPE so the doctor's gates have the data they need.
/// (Staging lease-sống (case thường trong lúc install) không bao giờ bị
/// gọi tên — truy vấn chỉ đưa bằng chứng; cổng pid/lock của doctor quyết
/// định. Ghim shape row để các cổng của doctor có dữ liệu cần.)
#[test]
fn covered_by_promoted_carries_lease_fields() {
    generation_token::test::promoted_coverage::covered_by_promoted_carries_lease_fields();
}
