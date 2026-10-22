//! Ensures status mapping preserves host order and invokes each configured probe.
//! Đảm bảo mapping giữ thứ tự host và gọi probe cho từng cấu hình.

use std::cell::RefCell;

use super::super::build_statuses;

#[test]
fn statuses_keep_input_order_and_probe_host_port_pairs() {
    let seen = RefCell::new(Vec::new());
    let statuses = build_statuses(
        vec![
            ("first.invalid".to_string(), 4315),
            ("second.invalid".to_string(), 4351),
        ],
        |host, port| {
            seen.borrow_mut().push((host.to_string(), port));
            host.starts_with("first")
        },
    );

    assert_eq!(statuses[0].host, "first.invalid");
    assert!(statuses[0].reachable);
    assert_eq!(statuses[1].host, "second.invalid");
    assert!(!statuses[1].reachable);
    assert_eq!(
        seen.into_inner(),
        vec![
            ("first.invalid".to_string(), 4315),
            ("second.invalid".to_string(), 4351)
        ]
    );
}
