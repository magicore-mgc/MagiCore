//! OCI descriptor model and builders tests.
//! Kiểm thử model và builder oci descriptor

use super::*;

#[test]
fn builder_methods_set_only_the_selected_optional_fields() {
    let descriptor = OciDescriptor::new("application/octet-stream".into(), 7, "sha256:abc".into())
        .with_data(vec![1, 2, 3])
        .with_urls(vec!["https://registry.example/blob".into()])
        .with_annotations(std::collections::HashMap::from([("k".into(), "v".into())]));

    assert_eq!(descriptor.media_type, "application/octet-stream");
    assert_eq!(descriptor.size, 7);
    assert_eq!(descriptor.digest, "sha256:abc");
    assert_eq!(descriptor.data, Some(vec![1, 2, 3]));
    assert_eq!(
        descriptor.urls,
        Some(vec!["https://registry.example/blob".to_string()])
    );
    assert_eq!(
        descriptor
            .annotations
            .as_ref()
            .and_then(|values| values.get("k"))
            .map(String::as_str),
        Some("v")
    );
}
