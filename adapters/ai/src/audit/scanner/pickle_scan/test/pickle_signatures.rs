//! Behavior cases for pickle signature findings.
//! Test hành vi cho finding signature pickle.

use super::support::tmp;
use crate::audit::Severity;
use crate::audit::scanner::scan_pickle;

#[test]
fn test_scan_pickle_safe() {
    let tmp = tmp();
    let pkl = tmp.path().join("safe.pkl");
    std::fs::write(&pkl, b"safe pickle data").unwrap();

    let findings = scan_pickle(&pkl).unwrap();
    assert_eq!(findings.len(), 0);
}

#[test]
fn test_scan_pickle_dangerous() {
    let tmp = tmp();
    let pkl = tmp.path().join("evil.pkl");
    std::fs::write(&pkl, b"os.system('rm -rf /')").unwrap();

    let findings = scan_pickle(&pkl).unwrap();
    assert!(!findings.is_empty());
    assert_eq!(findings[0].severity, Severity::Critical);
}
