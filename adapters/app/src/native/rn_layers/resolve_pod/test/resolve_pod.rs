#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! A pod without a lock checksum is rejected before any network fetch.
//! Pod thiếu checksum trong lock bị từ chối trước khi tải qua mạng.

use crate::native::rn_layers::resolve_pod::resolve_pod::resolve_pod;
use mgc_resolver::protocols::reactnative::{CocoaPodsProtocol, PodfileLock, PodfileLockPod};
use mgc_types::MgError;

#[tokio::test]
async fn missing_checksum_fails_before_fetch() {
    let lock = PodfileLock {
        pods: vec![PodfileLockPod {
            name: "ExamplePod".into(),
            version: "1.0.0".into(),
        }],
        checksums: Vec::new(),
    };
    let pod = &lock.pods[0];
    let protocol = CocoaPodsProtocol::from_env();
    let error = resolve_pod(pod, &lock, &protocol)
        .await
        .err()
        .expect("pod without lock checksum must be rejected");
    assert!(matches!(error, MgError::Integrity(_)));
}
