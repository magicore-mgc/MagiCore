# scaffold_ref

Owns the `ScaffoldRef` enum, its current validation rules, and typo suggestions. The current version check only requires a leading digit; it is not full SemVer validation. Ranges remain rejected with the existing message.

Quản lý enum `ScaffoldRef`, validation hiện tại và gợi ý lỗi gõ. Kiểm tra version hiện chỉ yêu cầu ký tự đầu là số, không phải kiểm tra SemVer đầy đủ. Range tiếp tục bị từ chối với thông báo hiện có.

The local `ScaffoldSpecError` owner supplies typed failures while public methods continue returning `anyhow::Result`.
