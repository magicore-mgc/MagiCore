//! Directory scans count recognized files.
//! Quét thư mục đếm file có định dạng được nhận diện.

use super::*;

#[tokio::test]
async fn test_audit_directory() {
    let tmp = tmp();
    std::fs::write(tmp.path().join("model1.bin"), b"fake").unwrap();
    std::fs::write(tmp.path().join("model2.pkl"), b"fake").unwrap();
    std::fs::write(tmp.path().join("readme.txt"), b"docs").unwrap();

    let report = audit_model(tmp.path()).await.unwrap();
    // Should scan .bin, .pkl (readme.txt skipped)
    // Quét file .bin, .pkl và bỏ qua readme.txt.
    assert!(report.scanned_files >= 2);
}
