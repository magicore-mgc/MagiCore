use super::super::parse_dump_package;

#[test]
fn dump_package_registry_remote_and_local() {
    let json = r#"{"name":"App","dependencies":[
            {"source":["registry","scope.lib",{"ranges":["1.0.0..<2.0.0"]}]},
            {"source":["remote","https://github.com/example/repo.git",{"from":"2.1.0"}]},
            {"source":["local","../sibling"]},
            {"source":["registry","weird.lib",{"weird":true}]}
        ]}"#;
    let deps = parse_dump_package(json).expect("test operation must succeed");
    assert_eq!(deps.len(), 2, "{deps:?}");
    assert_eq!(deps[0].dep_name(), "scope/lib");
    assert_eq!(deps[0].requirement_text(), "range:1.0.0..<2.0.0");
    assert_eq!(deps[1].dep_name(), "github.com/example/repo");
    assert_eq!(deps[1].requirement_text(), "from:2.1.0");
}

/// Real `swift package dump-package` shape (registry dep with a
/// lowerBound/upperBound range object) must parse to a usable dep.
/// Đảm bảo output registry thật có cặp lowerBound/upperBound thành dependency dùng được.
#[test]
fn dump_registry_range_object_parses() {
    let doc = r#"{"dependencies":[{"source":["registry","scope.lib",{"range":[{"lowerBound":"1.0.0","upperBound":"2.0.0"}]}]}]}"#;
    let deps = parse_dump_package(doc).expect("test operation must succeed");
    assert_eq!(deps.len(), 1, "dep must parse");
    assert_eq!(deps[0].dep_name(), "scope/lib");
    assert!(
        deps[0].requirement_text().contains("1.0.0"),
        "range carries bounds: {}",
        deps[0].requirement_text()
    );
}

/// Full realistic dump-package document (registry dep, range object,
/// traits array) must yield a usable dep — this is the exact shape
/// `swift package dump-package` emits.
/// Tài liệu dump-package đầy đủ phải tạo dependency dùng được theo đúng shape của toolchain.
#[test]
fn full_dump_document_yields_dep() {
    let doc = r#"{"dependencies":[{"registry":[{"identity":"scope.lib","productFilter":null,"requirement":{"range":[{"lowerBound":"1.0.0","upperBound":"2.0.0"}]},"traits":[{"name":"default"}]}]}]}"#;
    let deps = parse_dump_package(doc).expect("test operation must succeed");
    assert_eq!(deps.len(), 1);
    assert_eq!(deps[0].dep_name(), "scope/lib");
    assert!(deps[0].requirement_text().contains("1.0.0"));
}
