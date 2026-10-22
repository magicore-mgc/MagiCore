use std::ffi::OsString;

use super::super::process_observation;

#[test]
fn keeps_image_name_and_skips_argv_zero() {
    assert_eq!(
        process_observation(
            std::ffi::OsStr::new("node"),
            &[OsString::from("node"), OsString::from("--version")]
        ),
        ("node".to_string(), vec!["--version".to_string()])
    );
}
