# Attestation signing and storage / Ký và lưu attestation

Owns RFC 3339 timestamp generation, build-version stamping and signing/serialization/write of the attestation record. The project slot is created only at the same point as before, and filesystem/error ordering is preserved.

Quản lý sinh timestamp RFC 3339, gắn version build và ký/serialize/ghi record attestation. Project slot chỉ được tạo ở đúng bước hiện có; thứ tự filesystem/error được giữ.

`sign_and_store` remains an internal parent-visible helper; `attester_version` remains public. Signature and `anyhow` error behavior are unchanged.

`sign_and_store` tiếp tục là helper nội bộ chỉ parent thấy; `attester_version` vẫn public. Chữ ký và hành vi lỗi `anyhow` không đổi.
