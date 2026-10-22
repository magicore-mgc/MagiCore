//! Artifact fetch capability contract tests.
//! Test hợp đồng capability tải artifact.

use crate::capabilities::ArtifactFetcher;
use crate::capabilities::test_support::{MockCore, assert_unsupported};

#[test]
fn default_probe_fails_closed_with_original_contract() {
    assert_unsupported(
        MockCore.probe_artifact_fetcher().unwrap_err(),
        "web",
        "fetch",
        "nothing for mgc to fetch; the core's toolchain owns downloads",
    );
}
