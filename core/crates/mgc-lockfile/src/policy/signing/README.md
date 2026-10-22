# Payload signing / Ký payload

Signs only the decoded canonical payload digest with the supplied key pair and returns the existing inline Ed25519 block. Timestamp stays outside the signed payload. Key management remains owned by `mgc-crypto`.

Chỉ ký byte digest canonical bằng key pair truyền vào; trả inline Ed25519 block hiện có. Timestamp nằm ngoài signed payload; quản lý khóa thuộc `mgc-crypto`.
