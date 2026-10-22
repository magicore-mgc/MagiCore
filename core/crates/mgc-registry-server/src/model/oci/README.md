# OCI registry payloads / Payload registry OCI

Owns OCI manifests, descriptors, and the blob-upload response. Manifest/descriptor camelCase conversion, optional-value omission, defaulted annotations and the existing upload-response field names remain unchanged. The storage and upload route callers continue to use `crate::model::*`.

Quản lý OCI manifest, descriptor và response upload blob. Quy tắc camelCase của manifest/descriptor, bỏ giá trị tùy chọn, mặc định annotations và tên field upload response hiện tại được giữ nguyên. Caller storage và upload route tiếp tục dùng `crate::model::*`.
