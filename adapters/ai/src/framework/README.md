# AI framework owner

This owner exposes `AiFramework` and `detect_framework` through the same module path as before. It separates framework data/methods, name parsing, source-specific config readers and precedence.
Owner này xuất `AiFramework` và `detect_framework` qua cùng module path như trước. Nó tách dữ liệu/method framework, parse tên, reader theo từng nguồn config và thứ tự ưu tiên.

Detection remains best-effort: missing, unreadable, malformed, mistyped or unknown config returns no match and permits the existing fallback. No error type is added because this API returns `Option`.
Nhận diện tiếp tục best-effort: config thiếu, không đọc được, sai TOML/sai kiểu hoặc tên lạ thì không match và vẫn thử nguồn fallback hiện hành. Không tạo error type mới vì API này trả `Option`.
