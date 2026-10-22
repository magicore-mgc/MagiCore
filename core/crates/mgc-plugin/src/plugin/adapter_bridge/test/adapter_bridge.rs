// Fail-closed behavior tests for unsupported legacy bridge capabilities.
// Kiểm tra fail-closed cho capability bridge cũ chưa được hỗ trợ.
use super::super::{CacheUnsupported, TemplateUnsupported};
use crate::backend::CacheBackend;
use crate::template::TemplateGenerator;
use mgc_types::Ecosystem;
use std::path::Path;

fn assert_cache_error(error: mgc_types::error::MgError) {
    assert!(error.to_string().contains("cache backend not implemented"));
}

#[tokio::test]
async fn unsupported_template_keeps_fail_closed_error() {
    let template = TemplateUnsupported {
        ecosystem: Ecosystem::Web,
    };
    let error = template
        .generate(Path::new("."), "fixture")
        .await
        .expect_err("unsupported template generation must fail");
    assert!(
        error
            .to_string()
            .contains("template generator not implemented")
    );
}

#[tokio::test]
async fn unsupported_cache_operations_keep_fail_closed_errors() {
    let cache = CacheUnsupported {
        ecosystem: Ecosystem::Web,
    };
    assert_cache_error(
        cache
            .get("key")
            .await
            .expect_err("unsupported cache get should return an error"),
    );
    assert_cache_error(
        cache
            .put("key", &[])
            .await
            .expect_err("unsupported cache put should return an error"),
    );
    assert_cache_error(
        cache
            .claim(Path::new("."), &[])
            .await
            .expect_err("unsupported cache claim should return an error"),
    );
    assert_cache_error(
        cache
            .release(Path::new("."))
            .await
            .expect_err("unsupported cache release should return an error"),
    );
}
