//! Subject-local tests included in the original inspection module scope.
//! Test theo subject được include trong scope module inspection ban đầu.

include!("../policy_metadata/test/windows_job_denial_preserves_safe_invocation_evidence.rs");
include!(
    "../policy_metadata/test/windows_job_scan_ignores_global_processes_outside_job_membership.rs"
);
include!("../policy_metadata/test/windows_job_scan_fails_closed_for_unreadable_job_members.rs");
include!(
    "../member_capture/test/windows_job_member_capture_fails_closed_when_membership_cannot_be_verified.rs"
);
include!("../member_capture/test/windows_job_member_metadata_is_read_immediately_after_capture.rs");
include!("../member_capture/test/windows_job_member_without_immediate_metadata_fails_closed.rs");
include!(
    "../member_capture/test/windows_job_member_that_exits_during_metadata_capture_is_skipped.rs"
);
include!("../member_capture/test/windows_job_member_that_exits_before_capture_is_skipped.rs");
include!(
    "../member_capture/test/windows_job_snapshot_reenumerates_descendants_after_a_member_exits.rs"
);
include!(
    "../member_capture/test/windows_job_snapshot_reenumerates_members_after_a_clean_first_pass.rs"
);
include!(
    "../member_capture/test/windows_job_snapshot_fails_closed_when_membership_never_stabilizes.rs"
);
include!(
    "../scan_result/test/windows_job_completion_rejects_a_descendant_seen_after_the_snapshot.rs"
);
include!(
    "../member_capture/test/windows_job_snapshot_scans_a_captured_child_that_exits_after_membership_check.rs"
);
include!(
    "../policy_metadata/test/windows_job_scan_fails_closed_for_captured_child_missing_snapshot.rs"
);
include!("../policy_metadata/test/windows_go_policy_uses_only_observed_member_environment.rs");
include!("../policy_metadata/test/windows_job_device_control_children_follow_root_scope.rs");
