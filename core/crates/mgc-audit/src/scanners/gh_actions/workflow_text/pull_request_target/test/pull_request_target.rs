//! Test pull_request_target state and injection finding.
//! Kiểm tra trạng thái pull_request_target và finding injection.
#![allow(clippy::unwrap_used)]

use super::super::{injection_finding, observe_trigger};

#[test]
fn event_block_detects_only_the_existing_trigger_shape() {
    let mut block = None;
    let mut saw_target = false;
    observe_trigger(0, "on:", &mut block, &mut saw_target);
    observe_trigger(2, "pull_request_target:", &mut block, &mut saw_target);
    assert!(saw_target);

    let finding = injection_finding(
        "ci.yml",
        9,
        "run: echo ${{ github.event.pull_request.title }}",
        saw_target,
    )
    .unwrap();
    assert_eq!(finding.rule, "pull_request_target-injection");
    assert_eq!(finding.line, 9);
}

#[test]
fn unrelated_expressions_do_not_produce_injection_finding() {
    assert!(injection_finding("ci.yml", 3, "run: echo ${{ github.sha }}", true).is_none());
    assert!(
        injection_finding(
            "ci.yml",
            3,
            "run: echo ${{ github.event.pull_request }}",
            false
        )
        .is_none()
    );
}
