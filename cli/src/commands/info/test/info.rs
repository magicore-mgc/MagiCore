// Public no-Web command behavior tests.
// Test API công khai khi không bật Web.

#[cfg(not(feature = "web"))]
#[tokio::test]
async fn run_without_web_adapter_returns_the_existing_error() {
    let error = super::run("sample".to_string(), false)
        .await
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "package info currently requires the web core registry adapter, which is not included in this build"
    );
}

#[cfg(not(feature = "web"))]
#[tokio::test]
async fn versions_json_without_web_adapter_returns_the_existing_error() {
    let error = super::versions_json("sample").await.unwrap_err();
    assert_eq!(
        error.to_string(),
        "package info currently requires the web core registry adapter, which is not included in this build"
    );
}
