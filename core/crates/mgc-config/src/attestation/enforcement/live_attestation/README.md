# Live attestation / Chứng thực marker hiện hữu

Checks the attestation for a project whose core marker has already been validated. It preserves Off early-return, Warn compatibility and Require fail-closed behavior.

Kiểm tra attestation của project đã validate core marker. Giữ nguyên thoát sớm ở Off, tương thích ở Warn và fail-closed ở Require.

The typed error owner preserves user-facing diagnostic text and retains source errors for global identity access and verification failures. Tests are adjacent under `test/`.

Owner lỗi typed giữ nguyên diagnostic hiển thị và source error cho lỗi đọc global identity, verify. Test đặt gần module trong `test/`.
