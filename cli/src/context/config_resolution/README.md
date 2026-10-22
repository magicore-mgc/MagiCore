# Project config resolution / Phân giải cấu hình project

`config_resolution.rs` coordinates the established order: read persisted marker, reject selector conflicts, load `mgc.toml`, detect signatures, then allow an explicit core only for an unclaimed project. Branch behavior, canonical identity comparison, synthetic config and local conflict errors are delegated to same-name owners.

`config_resolution.rs` điều phối thứ tự hiện hành: đọc marker, từ chối selector xung đột, nạp `mgc.toml`, nhận diện chữ ký, cuối cùng mới cho phép chọn core tường minh nếu project chưa claim. Hành vi từng nhánh, so sánh identity chuẩn hóa, config tạm và lỗi conflict được giao cho owner cùng tên.

The coordinator preserves warning text and upstream error propagation. Its four tests cover marker precedence, reassignment rejection and the missing-root/explicit-core boundary. Test source is not yet run.

Coordinator giữ nguyên warning và chuỗi lỗi upstream. Bốn test kiểm tra độ ưu tiên marker, chặn đổi owner và ranh giới thiếu root/explicit core. Test chưa chạy.
