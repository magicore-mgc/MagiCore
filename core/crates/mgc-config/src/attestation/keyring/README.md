# Attestation keyring loading / Nạp keyring attestation

Loads an existing explicit keyring path or creates a new one while preserving the original validation-before-parent-creation/save sequence. It does not choose a global default path.

Nạp keyring tại path tường minh nếu đã có, hoặc tạo keyring mới và giữ nguyên thứ tự validate trước khi tạo parent/save. Owner không tự chọn global default path.

Existing invalid keyrings still return the existing `cannot load keyring` error chain. Nearest coverage checks that malformed persisted key material fails closed.

Keyring hiện có nhưng sai định dạng vẫn trả error chain `cannot load keyring`. Test gần nhất kiểm tra key material lỗi bị chặn.
