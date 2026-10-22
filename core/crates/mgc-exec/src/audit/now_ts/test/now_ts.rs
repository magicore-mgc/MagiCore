use super::super::now_ts;

#[test]
fn returns_a_plausible_unix_timestamp() {
    assert!(now_ts() > 1_700_000_000);
}
