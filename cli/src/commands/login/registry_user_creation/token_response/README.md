# Registry token response

EN: Parses bounded response bytes and accepts only a non-empty token with no whitespace or control bytes. It never renders token contents in errors.

VI: Owner này parse response trong giới hạn và chỉ nhận token không rỗng, không whitespace/control. Lỗi không in nội dung token.

The parent request owner enforces the streaming response limit; the decoder repeats the byte-size invariant for direct callers. Tests cover valid, malformed, missing and unsafe token values.

Owner request bên ngoài chặn kích thước khi đọc stream; decoder lặp lại invariant kích thước cho caller trực tiếp. Test bao phủ token hợp lệ, JSON lỗi, token thiếu và token không an toàn.
