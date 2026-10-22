//! Exact display contracts for local Web version errors.
//! Hợp đồng chuỗi hiển thị chính xác của lỗi version Web tại chỗ.

use super::*;
use std::path::Path;

#[test]
fn web_version_errors_keep_the_existing_diagnostic_text() {
    assert_eq!(
        web_scaffold_age_metadata_unavailable("react", &"offline").to_string(),
        "cannot enforce minimum-release-age for scaffold package 'react': registry metadata is unavailable: offline"
    );
    assert_eq!(
        web_scaffold_registry_version_unavailable("react", &"offline").to_string(),
        "cannot resolve the current registry version for scaffold package 'react': offline; retry when registry access is available"
    );
    assert_eq!(
        web_scaffold_security_config_unreadable(Path::new("/project"), &"bad config").to_string(),
        "cannot load web scaffold security policy from '/project': bad config"
    );
    assert_eq!(
        web_scaffold_store_policy_unreadable(Path::new("/project/store.db"), &"bad store")
            .to_string(),
        "cannot read web scaffold release policy from '/project/store.db': bad store"
    );
    assert_eq!(
        web_scaffold_store_policy_path_invalid(Path::new("/project/store.db")).to_string(),
        "web scaffold release policy database '/project/store.db' must be a regular file"
    );
    assert_eq!(
        web_scaffold_latest_tag_missing("react").to_string(),
        "npm metadata for 'react' has no latest dist-tag"
    );
    assert_eq!(
        web_scaffold_latest_tag_invalid("react").to_string(),
        "npm latest dist-tag for 'react' is invalid"
    );
    assert_eq!(
        web_scaffold_dist_tag_missing("react", "beta").to_string(),
        "npm metadata for 'react' has no 'beta' dist-tag"
    );
    assert_eq!(
        web_scaffold_dist_tag_invalid("react", "beta").to_string(),
        "npm 'beta' dist-tag for 'react' is invalid"
    );
    let version = mgc_types::Version::parse("1.2.3").unwrap();
    assert_eq!(
        web_scaffold_dist_tag_not_eligible("react", "beta", &version).to_string(),
        "npm 'beta' dist-tag for 'react' points to 1.2.3, which does not satisfy the configured minimum-release-age policy"
    );
    assert_eq!(
        web_scaffold_no_eligible_release("react").to_string(),
        "no stable release of 'react' satisfies the configured minimum-release-age policy"
    );
    assert_eq!(
        web_scaffold_no_eligible_release_in_range("react", "^1.2.0").to_string(),
        "no stable release of 'react' in scaffold range '^1.2.0' satisfies the configured minimum-release-age policy"
    );
}
