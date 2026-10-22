# OCI client / OCI client

The client facade keeps the public `mgc_oci::client::OciClient` path. Construction/authentication and catalog, blob, manifest, and model operations live in focused owners; `mod.rs` only wires and exports.

Facade giữ nguyên đường dẫn công khai `mgc_oci::client::OciClient`. Tạo client/auth và các thao tác catalog, blob, manifest, model nằm trong owner riêng; `mod.rs` chỉ nối module và export.

This split preserves current OCI behavior. It does not claim protocol conformance or change URL validation, response policy, memory limits, retries, or transport semantics.

Đợt tách này giữ hành vi OCI hiện tại. Nó không xác nhận tuân thủ protocol và không đổi validation URL, chính sách response, giới hạn RAM, retry hay semantics transport.
