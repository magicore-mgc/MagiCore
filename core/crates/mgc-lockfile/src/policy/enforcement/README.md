# Trust-policy enforcement / Áp chính sách trust

Applies `Off`, `Warn` or `Require` to `V4VerifyReport`. Tampered payload and invalid signatures always fail; `Require` additionally rejects unsigned or untrusted locks. Keep this after structural verification and preserve the existing error variants/messages.

Áp mode lên report; digest bị sửa và chữ ký sai luôn fail, còn `Require` từ chối lock unsigned/khóa không trusted. Giữ sau bước xác minh cấu trúc và giữ nguyên lỗi.
