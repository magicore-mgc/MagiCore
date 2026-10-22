//! Central-entry integrity tests — test toàn vẹn entry trung tâm.

use super::read_central_entry;
use crate::protocols::zip_reader::{read_zip_entries, test_support::build_zip};
use mgc_types::MgError;

#[test]
fn zip_crc_mismatch_fails_closed() {
    let mut zip = build_zip(&[("f.txt", b"payload".to_vec(), 0)]);
    // Corrupt one stored-payload byte; the 7-byte payload occupies [35..42].
    // Hỏng một byte payload lưu nguyên bản; payload 7 byte nằm tại [35..42].
    let idx = 30 + 5 + 3;
    zip[idx] ^= 0xFF;
    let err = read_zip_entries(&zip).expect_err("test operation must fail");
    assert!(matches!(err, MgError::Integrity(_)), "{err:?}");
}

#[test]
fn zip_central_entry_name_length_out_of_bounds_is_rejected() {
    let mut zip = build_zip(&[("f.txt", b"payload".to_vec(), 0)]);
    let central = zip
        .windows(4)
        .position(|window| window == b"PK\x01\x02")
        .expect("test operation must succeed");
    zip[central + 28..central + 30].copy_from_slice(&u16::MAX.to_le_bytes());

    let err = read_zip_entries(&zip).expect_err("test operation must fail");

    assert!(matches!(err, MgError::Other(_)), "{err:?}");
    assert!(err.to_string().contains("zip central directory truncated"));
}

#[test]
fn zip_total_budget_is_checked_before_payload_read() {
    let mut zip = build_zip(&[("f.txt", b"payload".to_vec(), 0)]);
    zip[..4].fill(0);
    let eocd = zip
        .windows(4)
        .rposition(|window| window == b"PK\x05\x06")
        .expect("test operation must succeed");
    let directory_start = u32::from_le_bytes(
        zip[eocd + 16..eocd + 20]
            .try_into()
            .expect("test operation must succeed"),
    ) as usize;
    let directory_size = u32::from_le_bytes(
        zip[eocd + 12..eocd + 16]
            .try_into()
            .expect("test operation must succeed"),
    ) as usize;
    let directory_end = directory_start + directory_size;

    let err = read_central_entry(&zip, directory_start, directory_end, 0)
        .expect_err("test operation must fail");

    assert!(matches!(err, MgError::Other(_)), "{err:?}");
    assert!(
        err.to_string()
            .contains("zip total uncompressed size exceeds the safety limit")
    );
}
