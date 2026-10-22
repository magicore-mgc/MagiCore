# Client transport and redacted Debug / Transport client và Debug đã che secret

Owns client configuration, builder methods, JSON transport and redacted Debug.
Quản lý cấu hình client, builder, JSON transport và Debug đã che bí mật.

The crate-visible JSON route remains `MgResult<Value>`; this owner delegates request failures to the local typed error owner. Diagnostics retain operation/status context while stripping caller-controlled URLs.
Route JSON crate-visible vẫn trả `MgResult<Value>`; owner chuyển lỗi request qua typed error owner cục bộ. Diagnostic giữ ngữ cảnh operation/status nhưng gỡ URL do caller kiểm soát.
