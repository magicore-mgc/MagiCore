# flag_policy

Validate mutually exclusive/unsupported Web create flags and map named presets to their current flags. Tailwind feature inclusion remains in the existing Web feature projection.

Kiểm tra các cờ Web create loại trừ/không hỗ trợ và ánh xạ preset có tên sang tập cờ hiện tại. Việc thêm Tailwind vào features vẫn do feature projection Web hiện có xử lý.

`flag_policy.rs` contains the implementation; `mod.rs` wires/re-exports it. The nearest tests cover rejected and accepted combinations plus each supported preset.

`flag_policy.rs` chứa implementation; `mod.rs` chỉ nối/export. Test gần nhất bao phủ tổ hợp bị từ chối/được chấp nhận và từng preset đang hỗ trợ.
