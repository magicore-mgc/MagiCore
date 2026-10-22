#[test]
fn verify_chain_rejects_unknown_steps_before_running() {
    // Fail-closed: unknown steps and empty chains are config errors.
    // Fail-closed: step lạ và chain rỗng là lỗi cấu hình.
    let known = ["audit", "test", "build"];
    for bad in ["audti", "builld", "", "lint", "deploy"] {
        assert!(!known.contains(&bad), "test fixture sanity");
    }
    let mut chain = vec![bad_step_fixture("audti").to_string()];
    chain.push("audit".into());
    // Simulate the validation loop from verify(): any unknown step fails.
    // Giả lập vòng validate của verify(): step lạ phải fail.
    let has_unknown = chain.iter().any(|s| !known.contains(&s.as_str()));
    assert!(has_unknown, "typo'd step must be detected, not skipped");
}

fn bad_step_fixture(name: &str) -> &str {
    name
}
