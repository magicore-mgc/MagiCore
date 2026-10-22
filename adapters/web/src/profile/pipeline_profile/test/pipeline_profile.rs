//! Tests for pipeline metric aggregation and bounded slow-item retention.
//! Kiểm tra tổng hợp metric pipeline và giới hạn danh sách item chậm.

use super::PipelineProfile;
use mgc_types::PackageId;

#[test]
fn records_totals_and_keeps_only_five_slowest_items() {
    let profile = PipelineProfile {
        enabled: true,
        ..Default::default()
    };
    for n in 1..=6 {
        let package = PackageId::parse(&format!("pkg-{n}@1.0.0")).expect("valid package ID");
        profile.record_download(&package, n * 10, n, 2, 3);
        profile.record_extract(&package, n + 1);
    }

    assert_eq!(
        profile
            .package_count
            .load(std::sync::atomic::Ordering::Relaxed),
        6
    );
    assert_eq!(
        profile
            .tarball_bytes
            .load(std::sync::atomic::Ordering::Relaxed),
        210
    );
    assert_eq!(
        profile
            .download_ms_total
            .load(std::sync::atomic::Ordering::Relaxed),
        21
    );
    assert_eq!(
        profile
            .download_ms_max
            .load(std::sync::atomic::Ordering::Relaxed),
        6
    );
    assert_eq!(
        profile
            .extract_ms_total
            .load(std::sync::atomic::Ordering::Relaxed),
        27
    );
    assert_eq!(
        profile
            .extract_ms_max
            .load(std::sync::atomic::Ordering::Relaxed),
        7
    );

    let downloads = profile.downloads_guard();
    assert_eq!(downloads.len(), 5);
    assert_eq!(downloads[0].0, 6);
    assert_eq!(downloads[0].2, 60);
    assert!(downloads[0].1.contains("queue_wait=2ms io=3ms"));
    drop(downloads);

    let extracts = profile.extracts_guard();
    assert_eq!(extracts.len(), 5);
    assert_eq!(extracts[0].0, 7);
}
