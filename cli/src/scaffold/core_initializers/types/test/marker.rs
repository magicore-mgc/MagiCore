//! Marker paths stay plain relative names.
//! Đường dẫn marker phải là tên tương đối thuần.

use super::super::CoreInitializer;

#[test]
fn marker_is_a_plain_relative_segment() {
    // A marker is joined onto the project root, so it must never be absolute or
    // climb out: a wrong marker would let a scaffold claim a directory it does
    // not own.
    // Marker được join vào root project, nên không bao giờ tuyệt đối hoặc đi
    // lên: marker sai sẽ để scaffold nhận thư mục nó không sở hữu.
    for marker in ["project.godot", "platformio.ini", "cdk.json", "west.yml"] {
        assert!(!marker.starts_with('/'));
        assert!(!marker.contains(".."));
        assert!(!marker.is_empty());
    }
}

#[test]
fn a_record_carries_its_ecosystem_and_framework() {
    let record = CoreInitializer {
        ecosystem: "game",
        framework: "godot",
        program: "godot",
        args_before: &["--headless"],
        args_after: &["--quit"],
        probe: &["--version"],
        install_hint: "https://godotengine.org/download",
        marker: "project.godot",
        destination: super::super::DestinationArg::Positional,
        board: None,
    };
    assert_eq!(record.ecosystem, "game");
    assert_eq!(record.framework, "godot");
}
