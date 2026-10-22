//! Bounded ZIP payload regressions — regression giới hạn payload ZIP.

use crate::protocols::zip_reader::{read_zip_entries, test_support::build_zip};
use mgc_types::MgError;

fn set_declared_uncompressed_size(zip: &mut [u8], declared: u32) {
    let central = zip
        .windows(4)
        .position(|window| window == b"PK\x01\x02")
        .expect("test operation must succeed");
    zip[central + 24..central + 28].copy_from_slice(&declared.to_le_bytes());
}

#[test]
fn zip_rejects_deflate_output_larger_than_declared_size() {
    let payload = vec![b'Z'; 4096];
    let mut zip = build_zip(&[("bomb.bin", payload, 8)]);
    set_declared_uncompressed_size(&mut zip, 1);

    let err = read_zip_entries(&zip).expect_err("test operation must fail");

    assert!(matches!(err, MgError::Integrity(_)), "{err:?}");
    assert!(err.to_string().contains("expected 1, got 2"));
}

#[test]
fn zip_rejects_deflate_output_smaller_than_declared_size() {
    let payload = vec![b'Z'; 4096];
    let mut zip = build_zip(&[("short.bin", payload, 8)]);
    set_declared_uncompressed_size(&mut zip, 4097);

    let err = read_zip_entries(&zip).expect_err("test operation must fail");

    assert!(matches!(err, MgError::Integrity(_)), "{err:?}");
    assert!(err.to_string().contains("expected 4097, got 4096"));
}

#[test]
fn zip_rejects_stored_size_mismatch_before_copy() {
    let mut zip = build_zip(&[("stored.bin", vec![b'S'; 4096], 0)]);
    set_declared_uncompressed_size(&mut zip, 4095);

    let err = read_zip_entries(&zip).expect_err("test operation must fail");

    assert!(matches!(err, MgError::Integrity(_)), "{err:?}");
    assert!(err.to_string().contains("expected 4095, got 4096"));
}
