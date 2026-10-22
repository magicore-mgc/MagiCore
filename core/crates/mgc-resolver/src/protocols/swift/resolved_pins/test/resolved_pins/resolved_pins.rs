use super::*;

#[test]
fn package_resolved_v1_and_v2_pins() {
    let v2 = r#"{"version":2,"pins":[
            {"identity":"scope.lib","kind":"registry","location":"registry+https://reg","state":{"version":"1.2.3"}},
            {"identity":"repo","kind":"remote","location":"https://github.com/example/repo.git","state":{"revision":"abc123","version":"2.0.0"}}
        ]}"#;
    let pins = parse_package_resolved(v2).expect("test operation must succeed");
    assert_eq!(pins.len(), 2);
    assert_eq!(pins[0].identity, "scope.lib");
    assert_eq!(pins[0].version.as_deref(), Some("1.2.3"));
    assert_eq!(pins[1].revision.as_deref(), Some("abc123"));

    let v1 = r#"{"version":1,"object":{"pins":[
            {"package":"Repo","repositoryURL":"https://github.com/example/repo.git","state":{"version":"2.0.0"}}
        ]}}"#;
    let pins = parse_package_resolved(v1).expect("test operation must succeed");
    assert_eq!(pins.len(), 1);
    assert_eq!(pins[0].identity, "Repo");
    assert_eq!(pins[0].version.as_deref(), Some("2.0.0"));
    assert_eq!(
        pins[0].location.as_deref(),
        Some("https://github.com/example/repo.git")
    );
}
