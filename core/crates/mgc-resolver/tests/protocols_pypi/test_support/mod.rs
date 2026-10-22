// Test-support module wiring — đăng ký và giới hạn truy cập fixture dùng chung.
#[path = "test_support.rs"]
mod implementation;

pub(super) use implementation::{
    file_entry, file_entry_at, index_json, mock_server, version_json, wheel_with_module,
    wheel_with_record,
};
