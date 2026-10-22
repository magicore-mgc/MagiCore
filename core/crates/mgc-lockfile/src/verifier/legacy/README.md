# Legacy sidecar verification / Xác minh sidecar legacy

Owns legacy missing-sidecar handling, signer-metadata downgrade protection, sidecar verification, and the mapping from tamper/verification errors into `VerificationStatus`. A present invalid sidecar must never become `Unsigned`.

Owner này xử lý thiếu sidecar legacy, chống downgrade khi metadata có signer, verify sidecar và ánh xạ lỗi tamper/verification sang `VerificationStatus`. Sidecar hiện diện nhưng sai không được thành `Unsigned`.
