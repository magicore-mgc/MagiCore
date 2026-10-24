//! Tests for generated and rewritten build HTML.
//! Test HTML build được tạo và rewrite.

use super::super::materialize_index_html;
use crate::bundler::BundlerConfig;
use tempfile::tempdir;

#[test]
fn process_assets_rewrites_root_index_html_to_built_bundle() {
    let tmp = tempdir().unwrap();
    let root = tmp.path();
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::create_dir_all(root.join("dist")).unwrap();
    std::fs::write(root.join("src/main.tsx"), "console.log('ok');").unwrap();
    std::fs::write(
        root.join("index.html"),
        "<!doctype html><html><body><div id=\"root\"></div><script type=\"module\" src=\"/src/main.tsx\"></script></body></html>",
    )
    .unwrap();

    let config = BundlerConfig {
        entry: root.join("src/main.tsx"),
        output_dir: root.join("dist"),
        minify: true,
        sourcemap: true,
        target: "es2020".to_string(),
        public_path: "/".to_string(),
        backend: false,
    };

    materialize_index_html(root, &config).unwrap();

    let generated = std::fs::read_to_string(root.join("dist/index.html")).unwrap();
    assert!(generated.contains("src=\"/main.js\""));
    assert!(!generated.contains("/src/main.tsx"));
}
