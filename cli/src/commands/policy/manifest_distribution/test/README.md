# Distribution tests / Test phân phối

Tests here lock down the URL-before-OCI channel plan without requiring network access. Network, signature, cache and fallback integration behavior remains in `cli/tests/policy_manifest_load.rs` and must still be run after repository-wide source closure.

Test tại đây khóa thứ tự URL trước OCI mà không cần mạng. Hành vi mạng, chữ ký, cache và fallback tích hợp vẫn ở `cli/tests/policy_manifest_load.rs` và phải chạy sau khi refactor source toàn repo hoàn tất.
