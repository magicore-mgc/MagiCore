# Optimizer dependency / Dependency optimizer

Owns the Cargo.toml mgc-optimizer path dependency mutation, conflict refusal, atomic write and persisted-value readback. It does not perform project identity checks or template publication.

Owner này sửa dependency path mgc-optimizer trong Cargo.toml, từ chối xung đột, ghi atomic và đọc xác nhận giá trị đã lưu. Không kiểm tra identity project hoặc publish template.
