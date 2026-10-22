# PyPI file record / Record file PyPI

Owns the serializable/deserializable file record consumed by PyPI storage, migration, trust, and simple-index routes. Rust field names, field types, derives, and current Serde naming behavior are preserved in this structure-only batch. Any protocol naming change requires a separately reviewed wire-contract fix.

Quản lý record file có thể serialize/deserialize được, dùng bởi PyPI storage, migration, trust và simple-index routes. Batch chỉ thay cấu trúc này giữ tên field Rust, kiểu field, derives và quy tắc đặt tên Serde hiện tại. Mọi thay đổi tên theo protocol cần được review riêng như một thay đổi wire-contract.
