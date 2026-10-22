// Filter refusal cases — preserve explicit fail-closed behavior.
// Ca từ chối filter — giữ hành vi fail-closed tường minh.
use super::super::{reject_filter_without_recursive, reject_unsupported_filter};
use super::parse_command;

#[test]
fn filter_on_publish_keeps_its_existing_refusal() {
    let publish = parse_command("publish", false, false);
    assert_eq!(
        reject_unsupported_filter(&publish).unwrap_err().to_string(),
        "--filter is not wired into 'publish' yet (the publish recursive pipeline does not accept filter). Refusing a silent no-op."
    );
}

#[test]
fn filter_without_recursive_keeps_its_existing_refusal() {
    assert_eq!(
        reject_filter_without_recursive().unwrap_err().to_string(),
        "--filter requires --recursive (it filters workspace targets). Use `mgc <cmd> --recursive --filter <glob>`."
    );
}
