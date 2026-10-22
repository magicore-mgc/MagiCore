use super::super::ParallelOptions;

#[test]
fn defaults_to_stopping_siblings_and_prefixing_output() {
    let options = ParallelOptions::default();
    assert!(options.kill_others);
    assert!(options.prefix_output);
}
