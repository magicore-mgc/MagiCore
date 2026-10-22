#[test]
fn windows_job_completion_rejects_a_descendant_seen_after_the_snapshot() {
    assert!(!super::windows_job_has_active_descendants(100, &[100]));
    assert!(super::windows_job_has_active_descendants(100, &[100, 101]));
}
