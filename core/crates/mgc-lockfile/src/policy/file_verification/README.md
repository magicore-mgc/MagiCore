# On-disk lock verification / Xác minh lock trên đĩa

Owns the end-to-end v4 file route: bounded byte read, UTF-8 conversion, document parsing, structural verification, then trust-policy enforcement. The order and parser/read errors are part of the existing contract.

Sở hữu luồng verify file V4: đọc byte có giới hạn, UTF-8, parse, verify cấu trúc rồi enforce policy. Thứ tự và lỗi reader/parser là contract hiện tại.
