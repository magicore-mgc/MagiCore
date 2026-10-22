// SBOM mapping — retain format, output path and project identity.
// Mapping SBOM — giữ định dạng, đường dẫn output và identity dự án.
use super::super::common_to_dispatch;
use crate::commands::definitions::commands::common::CommonCommands;
use crate::dispatch::types::CommonCommand;
use std::path::PathBuf;

#[test]
fn maps_sbom_output_and_project_metadata() {
    let output = PathBuf::from("sbom.json");
    let dir = PathBuf::from("project");
    match common_to_dispatch(CommonCommands::Sbom {
        format: Some("spdx-json".to_owned()),
        output: Some(output.clone()),
        name: Some("sample".to_owned()),
        version: Some("2.4.1".to_owned()),
        dir: Some(dir.clone()),
    }) {
        Some(CommonCommand::Sbom {
            format,
            output: actual_output,
            name,
            version,
            dir: actual_dir,
        }) => {
            assert_eq!(format.as_deref(), Some("spdx-json"));
            assert_eq!(actual_output, Some(output));
            assert_eq!(name.as_deref(), Some("sample"));
            assert_eq!(version.as_deref(), Some("2.4.1"));
            assert_eq!(actual_dir, Some(dir));
        }
        _ => panic!("SBOM command mapped to a different variant"),
    }
}
