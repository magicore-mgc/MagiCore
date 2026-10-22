#[test]
fn adb_device_list_requires_a_ready_device_record() {
    let header_only = "List of devices attached\n\n";
    let unauthorized = "List of devices attached\nemulator-5554 unauthorized\n";
    let offline = "List of devices attached\nemulator-5554 offline\n";
    let ready = "List of devices attached\nemulator-5554 device product:sdk_gphone64\n";

    assert!(!super::adb_output_has_ready_device(header_only));
    assert!(!super::adb_output_has_ready_device(unauthorized));
    assert!(!super::adb_output_has_ready_device(offline));
    assert!(super::adb_output_has_ready_device(ready));
}
