//! Maven coordinate parser and cache containment tests.
//! Test parse tọa độ Maven và containment cache.

use crate::protocols::maven::registry::registry::MavenProtocol;

#[test]
fn valid_group_artifact_coordinate_is_preserved() {
    assert_eq!(
        MavenProtocol::split_coordinate("dev.mgc:probe").expect("test operation must succeed"),
        ("dev.mgc".to_string(), "probe".to_string())
    );
}

#[test]
fn malformed_and_path_changing_coordinates_are_rejected() {
    for input in [
        "",
        "group",
        ":artifact",
        "group:",
        "dev..mgc:probe",
        "dev.mgc:../probe",
        "dev.mgc:probe/escape",
        "dev.mgc:probe:extra",
    ] {
        assert!(
            MavenProtocol::split_coordinate(input).is_err(),
            "must reject {input:?}"
        );
    }
}
