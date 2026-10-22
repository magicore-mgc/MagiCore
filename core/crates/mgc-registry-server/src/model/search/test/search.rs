//! Focused npm search response wire-format tests.
//! Test riêng định dạng response tìm kiếm npm.

use crate::model::{SearchLinks, SearchPackage, SearchPublisher, SearchResult, SearchResultItem};

#[test]
fn test_search_response_keeps_nested_package_and_search_score() {
    let response = SearchResult {
        objects: vec![SearchResultItem {
            package: SearchPackage {
                name: "pkg".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                keywords: None,
                date: "2026-10-20".to_string(),
                links: SearchLinks {
                    npm: Some("https://registry.example/pkg".to_string()),
                    homepage: None,
                    repository: None,
                    bugs: None,
                },
                publisher: SearchPublisher {
                    username: "maintainer".to_string(),
                    email: None,
                },
            },
            score: 0.8,
            search_score: 0.9,
        }],
        total: 1,
        time: "2026-10-20T00:00:00.000Z".to_string(),
    };
    let json = serde_json::to_value(response).unwrap();

    assert_eq!(json["objects"][0]["package"]["name"], "pkg");
    assert_eq!(json["objects"][0]["searchScore"], 0.9);
}
