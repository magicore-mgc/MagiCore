# Configured project owner / Owner project có cấu hình

`configured_project.rs` loads `mgc.toml`, reconciles its ecosystem with a validated marker or explicit selector, and records a missing marker from the existing config. Read-only marker persistence keeps the established warning and uses the config identity.

`configured_project.rs` nạp `mgc.toml`, đối chiếu ecosystem với marker hoặc selector đã xác thực, rồi ghi marker còn thiếu dựa trên config hiện hữu. Nếu thư mục chỉ đọc, owner giữ warning cũ và dùng identity trong config.

Nearest tests cover legacy adoption, matching override adoption and read-only fallback. Runtime test remains pending.

Test gần owner kiểm tra nhận identity legacy, override trùng khớp và fallback thư mục chỉ đọc. Runtime test đang chờ.
