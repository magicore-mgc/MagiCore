# Registry request owner / Owner của request registry

This owner resolves the configured Web registry, builds the existing npm search endpoint, sends the HTTP request, and decodes the response envelope. Pure envelope parsing stays independently testable; request policy is unchanged.

Owner này lấy Web registry đã cấu hình, tạo endpoint npm search hiện tại, gửi HTTP request và giải mã envelope. Việc parse envelope thuần được test riêng; policy request giữ nguyên.
