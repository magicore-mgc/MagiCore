# Local-entry payload owner

## English

Reads the local header, calculates the payload region with checked overflow/bounds, then supports stored (0) and raw-DEFLATE (8) data. DEFLATE output is read through an `expected + 1` bound, and both codecs must exactly match the central-directory uncompressed size before CRC verification. Initial allocation stays capped at 64 KiB. The roundtrip remains covered through the public read pipeline; metadata mismatch cases are adjacent.

## Tiếng Việt

Đọc local header, tính vùng payload bằng checked overflow/bounds rồi hỗ trợ store (0) và raw DEFLATE (8). Output DEFLATE được đọc có giới hạn `expected + 1`; cả hai codec phải khớp chính xác kích thước uncompressed trong central-directory trước khi xác minh CRC. Dung lượng cấp phát ban đầu tối đa 64 KiB. Roundtrip tiếp tục được bao phủ qua public read pipeline; test metadata lệch đặt cạnh source.
