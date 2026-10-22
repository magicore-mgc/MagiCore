# Native API operations / Các operation API native

Implements the existing native metadata/list/existence API over the shared HF client.
Cài đặt API metadata/list/exists hiện tại trên client HF dùng chung.

This owner preserves the existing trait methods and endpoint formats, and maps local typed failures to the shared `MgError` at the trait boundary. URL-bearing diagnostics are redacted.
Owner giữ trait method và endpoint, ánh xạ lỗi có kiểu cục bộ sang `MgError` chung tại trait boundary. Diagnostic có URL được che.
