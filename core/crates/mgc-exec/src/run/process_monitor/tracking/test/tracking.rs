use super::super::process_tree_tracking_token;

#[test]
fn successive_process_tree_tokens_are_distinct() {
    let first = process_tree_tracking_token();
    let second = process_tree_tracking_token();

    assert!(!first.is_empty());
    assert_ne!(first, second);
}
