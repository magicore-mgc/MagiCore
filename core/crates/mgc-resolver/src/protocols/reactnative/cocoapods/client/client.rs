//! Fetch a podspec and verify bytes before parsing the JSON document.
//! Tải podspec và xác minh byte trước khi parse tài liệu JSON.

use super::super::{CocoaPodsProtocol, VerifiedPodSpec};
use super::error::CocoaPodsClientError;
use sha1::Digest as _;
use sha1::Sha1;

pub(in crate::protocols::reactnative::cocoapods) async fn fetch_verified_spec(
    protocol: &CocoaPodsProtocol,
    name: &str,
    version: &str,
    expected_sha1: &str,
) -> Result<VerifiedPodSpec, CocoaPodsClientError> {
    let expected = expected_sha1.trim().to_ascii_lowercase();
    if expected.len() != 40
        || !expected
            .chars()
            .all(|character| character.is_ascii_hexdigit())
    {
        return Err(CocoaPodsClientError::InvalidChecksum {
            name: name.to_string(),
            expected: expected_sha1.to_string(),
        });
    }
    let url = super::super::spec_url::cocoapods_spec_url(&protocol.cdn, name, version);
    let response =
        protocol
            .client
            .get(&url)
            .await
            .map_err(|source| CocoaPodsClientError::RequestFailed {
                url: url.clone(),
                source,
            })?;
    let status = response.status();
    let bytes = response
        .bytes()
        .await
        .map_err(|source| CocoaPodsClientError::BodyReadFailed {
            url: url.clone(),
            source: source.into(),
        })?;
    if !status.is_success() {
        return Err(CocoaPodsClientError::HttpStatus {
            url,
            status: status.to_string(),
        });
    }
    let actual = hex::encode(Sha1::digest(&bytes));
    if actual != expected {
        return Err(CocoaPodsClientError::ChecksumMismatch {
            name: name.to_string(),
            version: version.to_string(),
            expected,
            actual,
        });
    }
    let spec =
        serde_json::from_slice(&bytes).map_err(|source| CocoaPodsClientError::InvalidJson {
            name: name.to_string(),
            version: version.to_string(),
            source,
        })?;
    Ok(VerifiedPodSpec { sha1: actual, spec })
}
