//! Timestamp parsing cases / các trường hợp parse timestamp.

use crate::provider::parse_npm_time;

#[test]
fn parses_a_second_npm_timestamp_and_rejects_missing_time_parts() {
    assert_eq!(parse_npm_time("2021-12-31T23:59:59Z"), Some(1640995199));
    assert_eq!(parse_npm_time("2021-12-31T23:59"), None);
}

#[test]
fn validates_calendar_days_and_gregorian_leap_years() {
    assert_eq!(parse_npm_time("2024-02-29T00:00:00Z"), Some(1709164800));
    assert_eq!(parse_npm_time("2000-02-29T00:00:00Z"), Some(951782400));
    assert_eq!(parse_npm_time("2023-02-29T00:00:00Z"), None);
    assert_eq!(parse_npm_time("2100-02-29T00:00:00Z"), None);
    assert_eq!(parse_npm_time("2024-02-00T00:00:00Z"), None);
    assert_eq!(parse_npm_time("2024-02-30T00:00:00Z"), None);
    assert_eq!(parse_npm_time("2024-04-31T00:00:00Z"), None);
}

#[test]
fn rejects_pre_epoch_and_unrepresentable_years_without_overflow() {
    assert_eq!(parse_npm_time("0000-01-01T00:00:00Z"), None);
    assert_eq!(parse_npm_time("1969-12-31T23:59:59Z"), None);
    assert_eq!(parse_npm_time("1970-01-01T00:00:00Z"), Some(0));
    assert_eq!(parse_npm_time("1970-01-01T00:00:01Z"), Some(1));
    assert_eq!(parse_npm_time("18446744073709551615-12-31T23:59:59Z"), None);
}
