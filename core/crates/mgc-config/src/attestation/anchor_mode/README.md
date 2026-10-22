# Anchor enforcement mode / Chế độ enforcement anchor

Owns `AnchorMode`, `ANCHOR_MODE_ENV`, accepted token parsing and the current default-to-Warn environment behavior. It does not access project or identity storage.

Quản lý `AnchorMode`, `ANCHOR_MODE_ENV`, parse token được chấp nhận và hành vi mặc định Warn theo environment. Owner không truy cập project hoặc identity storage.

Malformed values continue to fall back to Warn; the existing parser tokens are preserved. The nearest unit test covers aliases and rejected values.

Giá trị sai định dạng tiếp tục fallback về Warn; token parser hiện có được giữ nguyên. Test gần nhất bao phủ alias và giá trị bị từ chối.
