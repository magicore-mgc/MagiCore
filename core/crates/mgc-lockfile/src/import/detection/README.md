# Legacy lockfile detection

Detects supported lockfile files without parsing or importing them, and reports a stale-lockfile trust-downgrade risk next to `mgc.lock`.

Nhận diện file lock hỗ trợ mà không parse/import, đồng thời báo nguy cơ trust-downgrade khi lockfile cũ nằm cạnh `mgc.lock`.

The CLI install hint and post-import warning use these stable functions. Detection order remains the order in the public `ALL` constant.

Gợi ý trong CLI install và cảnh báo sau import dùng các hàm ổn định này. Thứ tự phát hiện giữ theo hằng `ALL` công khai.
