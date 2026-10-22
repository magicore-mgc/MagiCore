# HTTP proxy configuration / Cấu hình proxy HTTP

This folder preserves `mgc_http::proxy::ProxyConfig` and separates configuration data, environment loading, bypass matching, registry overrides, selection, reqwest application and local construction errors.

Folder này giữ `mgc_http::proxy::ProxyConfig` và tách dữ liệu cấu hình, đọc environment, so khớp bypass, override registry, chọn proxy, áp dụng vào reqwest và lỗi khởi tạo cục bộ.

Known limits remain: raw registry-prefix matching, case-sensitive NO_PROXY matching, no port/CIDR parsing and no HttpClient integration. This batch preserves them; it does not claim to fix them.

Giới hạn hiện tại: so khớp prefix registry thô, NO_PROXY phân biệt hoa thường, không parse port/CIDR và chưa tích hợp vào HttpClient. Batch này giữ nguyên, không tuyên bố đã sửa.
