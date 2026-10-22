// Only the closed version-query forms are eligible for signed exceptions.
// Chỉ dạng truy vấn version trong danh sách đóng mới đủ điều kiện exception.
#[test]
fn exception_only_accepts_closed_read_only_probes() {
    use crate::policy::is_read_only_probe;

    for args in [vec!["--version".to_string()], vec!["-v".to_string()]] {
        assert!(is_read_only_probe("npm", &args));
    }
    for args in [
        vec!["exec".to_string(), "x".to_string()],
        vec!["run".to_string(), "dev".to_string()],
        vec!["view".to_string(), "x".to_string()],
        vec!["install".to_string(), "x".to_string()],
    ] {
        assert!(!is_read_only_probe("npm", &args));
    }
    assert!(!is_read_only_probe("fakepm", &["--version".to_string()]));
}
