// Keep the Cargo target's registration surface flat and preserve test order.
// Giữ đăng ký test phẳng trong Cargo target và bảo toàn thứ tự.

include!("concurrent_same_tarball_writers_publish_one_complete_entry/mod.rs");
include!("conflicting_tarball_winner_is_not_overwritten/mod.rs");
