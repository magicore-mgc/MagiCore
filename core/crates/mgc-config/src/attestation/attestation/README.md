# Project attestation operation / Tác vụ attest project

Coordinates canonical core validation, live-marker match, idempotent verification, config digest, explicit/default key selection, bounded previous-key rotation and signing/storage. The owner preserves the original order and diagnostic strings.

Điều phối kiểm tra core canonical, khớp marker, verify idempotent, digest config, chọn key chỉ định/default, rotation một thế hệ và ký/lưu. Owner giữ nguyên thứ tự cùng diagnostic.

The public `attest` signature remains under `mgc_config::attestation::attest`; errors stay `anyhow`. Nearest coverage rejects unknown cores before touching project/keyring state. End-to-end rotation/idempotency coverage remains in `tests/core_attestation.rs`.

Signature public `attest` vẫn ở `mgc_config::attestation::attest`; lỗi tiếp tục dùng `anyhow`. Test gần nhất từ chối core lạ trước khi chạm project/keyring. Test end-to-end rotation/idempotency giữ ở `tests/core_attestation.rs`.
