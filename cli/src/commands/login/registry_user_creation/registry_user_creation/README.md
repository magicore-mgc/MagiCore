# Registration request coordinator

EN: Builds the existing registration JSON, sends the PUT request through the shared safe registry client, enforces status/body limits, and passes bytes to the token decoder.

VI: Owner này dựng JSON đăng ký hiện tại, gửi PUT qua shared registry client an toàn, chặn status/body vượt giới hạn rồi chuyển bytes cho token decoder.

The client must not follow credential-bearing redirects. Request failures remain sanitized to the existing stable diagnostic. Tests cover JSON construction only; HTTP behavior still needs controlled runtime verification.

Client không được theo redirect mang credential. Lỗi request tiếp tục được che theo diagnostic ổn định hiện tại. Test mới kiểm tra dựng JSON; hành vi HTTP cần runtime verification có kiểm soát.
