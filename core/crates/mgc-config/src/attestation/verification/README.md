# Live project attestation verification / Verify attestation project

Owns the ordered verification path: locate slot, parse the closed-schema record, validate live core marker, read secure config bytes, compare digest, then verify against the current/recorded prior public keys.

Quản lý thứ tự verify: tìm slot, parse record schema đóng, validate core marker, đọc config an toàn, so digest rồi verify với public key hiện tại/key cũ được lưu.

Mismatch reasons and `AttestationStatus`/`anyhow` boundaries stay unchanged. Existing integration cases continue to cover tampered marker/config/signature and rotation; the nearest test covers absent state.

Lý do mismatch và ranh giới `AttestationStatus`/`anyhow` không đổi. Integration hiện có tiếp tục bao phủ marker/config/signature bị sửa và rotation; test gần nhất kiểm tra state vắng.
