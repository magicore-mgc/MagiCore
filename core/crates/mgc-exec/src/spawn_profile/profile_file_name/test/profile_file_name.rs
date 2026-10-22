use super::super::profile_file_name;

#[test]
fn normalizes_each_tuple_component_and_keeps_the_json_suffix() {
    assert_eq!(
        profile_file_name("next/web", "dev server", "darwin/arm64"),
        "next-web-dev-server-darwin-arm64.json"
    );
    assert_eq!(profile_file_name("", "build", "linux"), "-build-linux.json");
}
