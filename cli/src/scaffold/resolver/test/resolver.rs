#![allow(clippy::unwrap_used)]

use crate::scaffold::resolver::spec_to_layer_path;
use crate::scaffold::spec::{CoreKind, ScaffoldRef, ScaffoldSpec, parse_scaffold_spec};

#[test]
fn maps_each_supported_core_to_its_expected_artifact_layer_prefix() {
    let cases = [
        (CoreKind::Web, "web/frontend"),
        (CoreKind::Ai, "ai"),
        (CoreKind::App, "app"),
        (CoreKind::Lib, "lib"),
        (CoreKind::Game, "game"),
        (CoreKind::Iot, "iot"),
        (CoreKind::Cicd, "cicd"),
        (CoreKind::Cloud, "cloud"),
        (CoreKind::Hardware, "hardware"),
    ];
    for (core, prefix) in cases {
        let spec = ScaffoldSpec {
            core,
            name: "User Template".to_string(),
            normalized_name: "normalized-template".to_string(),
            requested_ref: ScaffoldRef::Default,
        };
        assert_eq!(
            spec_to_layer_path(&spec),
            format!("{prefix}/normalized-template")
        );
    }
}

#[test]
fn path_mapping_uses_the_spec_normalized_name() {
    let spec = parse_scaffold_spec(CoreKind::App, "React_Native").unwrap();
    assert_eq!(spec_to_layer_path(&spec), "app/react-native");
}
