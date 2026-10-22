// Register streaming checksum cases — Đăng ký test checksum theo luồng.
// Keep test implementations beside the checksum owner — Đặt test gần owner checksum.
use super::*;

include!("streaming_tarball_integrity_matches_buffered_integrity/mod.rs");
include!("streaming_tarball_integrity_returns_reader_errors/mod.rs");
