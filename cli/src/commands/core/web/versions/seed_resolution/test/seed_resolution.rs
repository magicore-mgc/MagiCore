//! Seed range preservation tests.
//! Test giữ nguyên range seed.

use super::super::super as versions;

#[tokio::test]
async fn scaffold_keeps_a_user_requested_older_package_range() {
    let selected = versions::resolve_seed_dependency_version("next", Some("^15.2.0"), None)
        .await
        .expect("an explicit compatible user range must be retained");
    assert_eq!(selected, "^15.2.0");
}
