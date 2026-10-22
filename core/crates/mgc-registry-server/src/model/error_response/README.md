# Registry error response / Error response registry

Owns the serializable HTTP error payload and its public `new`/`with_details` builders. The `details` field remains omitted when absent; the type is still Serialize-only. Existing callers keep using `mgc_registry_server::model::ErrorResponse`.

Quản lý payload lỗi HTTP có thể serialize và các builder công khai `new`/`with_details`. Field `details` tiếp tục bị lược bỏ khi không có; kiểu này vẫn chỉ derive Serialize. Caller hiện tại tiếp tục dùng `mgc_registry_server::model::ErrorResponse`.
