//! Live Packagist (Composer) e2e through the real resolver engine.
//! E2E Packagist (Composer) thật qua engine resolver thật.
//!
//! The composer protocol test doubles live in `mgc-resolver` (hermetic,
//! mockito). This target is the live lane: it resolves `monolog/monolog`
//! against `https://repo.packagist.org/p2/...`, checks the sha1 integrity
//! marker, downloads the published dist zip and verifies it with the engine's
//! own `verify`, so no mock stands in for the registry.
//! Bản giả lập protocol composer nằm trong `mgc-resolver` (hermetic,
//! mockito). Lane này chạy THẬT: resolve `monolog/monolog` trên
//! `https://repo.packagist.org/p2/...`, kiểm marker toàn vẹn sha1, tải dist
//! zip đã công bố và xác minh bằng `verify` của engine — không có mock thay
//! registry.
//!
//! Gate: the existing repo convention — a binary-dependent e2e asserts
//! `CARGO_BIN_EXE_mgc`, and network lanes skip honestly when the registry
//! host is unreachable (a network-less run proves nothing) unless the lane is
//! declared required.
//! Gate: theo quy ước sẵn có — e2e cần binary kiểm `CARGO_BIN_EXE_mgc`, lane
//! mạng skip trung thực khi host registry không tới được (chạy không mạng
//! không chứng minh gì), trừ khi lane được khai báo required.

#![allow(clippy::unwrap_used)]

use mgc_resolver::protocols::{ComposerProtocol, RegistryProtocol};
use std::time::Duration;

const PACKAGE: &str = "monolog/monolog";
const CONSTRAINT: &str = "^3.0";

/// Live lanes need a real binary in the test run, exactly like the other
/// network e2e targets.
/// Lane live cần binary thật trong lượt test, giống các target e2e mạng khác.
fn require_binary() {
    std::env::var_os("CARGO_BIN_EXE_mgc")
        .expect("CARGO_BIN_EXE_mgc unavailable — run via cargo test");
}

/// Bounded reachability for repo.packagist.org.
/// Kiểm tra tới được repo.packagist.org có giới hạn thời gian.
fn packagist_reachable() -> bool {
    use std::net::ToSocketAddrs;
    let Ok(mut addrs) = "repo.packagist.org:443".to_socket_addrs() else {
        return false;
    };
    let Some(addr) = addrs.next() else {
        return false;
    };
    std::net::TcpStream::connect_timeout(&addr, Duration::from_secs(5)).is_ok()
}

/// Skip honestly without network; a required lane fails instead.
/// Skip trung thực khi không mạng; lane required thì fail.
fn reachable_or_skip() -> bool {
    if packagist_reachable() {
        return true;
    }
    let required = std::env::var("MGC_E2E_COMPOSER_NETWORK")
        .map(|value| value == "required")
        .unwrap_or(false);
    if required {
        panic!("MGC_E2E_COMPOSER_NETWORK=required but repo.packagist.org is unreachable");
    }
    let test_name = std::thread::current()
        .name()
        .unwrap_or("unknown-test")
        .to_string();
    eprintln!("SKIP (environment-unverified) test={test_name}: repo.packagist.org unreachable");
    false
}

fn protocol() -> ComposerProtocol {
    ComposerProtocol::from_env()
}

#[tokio::test]
async fn composer_resolves_monolog_from_the_live_packagist_registry() {
    require_binary();
    if !reachable_or_skip() {
        return;
    }

    let protocol = protocol();
    // `metadata_url` is crate-private; the live URL shape is exercised by
    // the resolve below against the configured repository.
    // `metadata_url` là private trong crate; dạng URL thật được kiểm qua
    // resolve bên dưới trên repository đã cấu hình.

    let entry = protocol
        .resolve(PACKAGE, CONSTRAINT)
        .await
        .expect("live Packagist resolve must succeed");

    // Resolved package id is the Composer `vendor/package`, and the version
    // is a real 3.x release with a Packagist dist zip.
    // ID package là `vendor/package` Composer; version là release 3.x thật
    // kèm dist zip của Packagist.
    assert_eq!(entry.name, PACKAGE);
    let (major, _) = entry
        .version
        .split_once('.')
        .expect("resolved version has major.minor");
    assert_eq!(major, "3", "resolved {}", entry.version);
    assert!(
        entry.artifact_url.starts_with("https://"),
        "dist url: {}",
        entry.artifact_url
    );
    assert!(
        entry
            .artifact_url
            .to_ascii_lowercase()
            .contains("/monolog/"),
        "dist url should point at the monolog repository: {}",
        entry.artifact_url
    );

    // Integrity: either the published `dist.shasum` (sha1) or the VCS
    // `dist.reference` must be recorded; never silent.
    // Toàn vẹn: phải ghi `dist.shasum` (sha1) hoặc `dist.reference`; không
    // bao giờ im lặng.
    let has_sha1 = entry
        .extra_markers
        .iter()
        .any(|marker| marker.starts_with("checksum-sha1:"));
    let has_reference = entry
        .extra_markers
        .iter()
        .any(|marker| marker.starts_with("dist-reference:"));
    assert!(
        has_sha1 || has_reference,
        "no integrity marker in {:?}",
        entry.extra_markers
    );

    // Dependencies: `require` edges only, platform packages skipped.
    // Phụ thuộc: chỉ cạnh `require`, bỏ package platform.
    assert!(
        entry.deps.iter().all(|(name, _)| name != "php"),
        "platform php must never be a registry edge: {:?}",
        entry.deps
    );
    assert!(
        entry.deps.iter().all(|(name, _)| !name.contains(':')),
        "runtime graph must not include require-dev edges: {:?}",
        entry.deps
    );
    assert!(
        entry
            .extra_markers
            .iter()
            .any(|marker| marker.starts_with("require-dev:")),
        "require-dev edges must be recorded as markers: {:?}",
        entry.extra_markers
    );

    // Download the published dist and verify it through the engine.
    // Tải dist đã công bố và xác minh qua engine.
    let bytes = protocol
        .download(&entry)
        .await
        .expect("live dist download must succeed");
    assert!(!bytes.is_empty(), "dist archive is empty");
    protocol
        .verify(&entry, &bytes)
        .expect("dist must verify against the recorded integrity source");

    // Report the exact release the live registry served.
    // Ghi lại đúng bản phát hành mà registry thật trả về.
    eprintln!(
        "COMPOSER_LIVE_RESOLVED {PACKAGE} {CONSTRAINT} -> {} ({} bytes, markers={:?})",
        entry.version,
        bytes.len(),
        entry.extra_markers
    );
}
