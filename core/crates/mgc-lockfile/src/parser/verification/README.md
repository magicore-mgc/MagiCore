# Signature verification / Xác minh chữ ký

Owns the order-sensitive legacy sidecar verification transaction: bounded lock read, schema parse, bounded sidecar read, sidecar decode, lock hash, signer fingerprint, signature decode, and Ed25519 verification. Local verification failures map to the existing tamper/verification error variants.

Owner này quản lý thứ tự xác minh sidecar legacy: đọc lock có giới hạn, parse schema, đọc sidecar có giới hạn, decode sidecar, hash lock, fingerprint signer, decode chữ ký và verify Ed25519. Lỗi verify cục bộ ánh xạ về variant tamper/verification hiện tại.
