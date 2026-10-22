# EOCD signature finder

## English

Scans the final 65,535-byte comment window plus the 22-byte EOCD record for `PK\x05\x06`. Existing diagnostics for too-short input or missing signature are preserved. Coverage is exercised by the adjacent end-record regression.

## Tiếng Việt

Quét vùng comment cuối 65.535 byte cộng record EOCD 22 byte để tìm `PK\x05\x06`. Giữ diagnostic khi input quá ngắn hoặc thiếu signature. Regression end-record kế cận bao phủ hành vi này.
