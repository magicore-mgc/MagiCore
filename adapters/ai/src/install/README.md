# AI model installation

This owner preserves the public AI model-install API while keeping the root module limited to declarations and stable re-exports. Lifecycle orchestration, public result types, download behavior, and checksum verification each have an explicit owner.

Owner này giữ API cài model AI hiện tại, còn module gốc chỉ khai báo và export ổn định. Điều phối vòng đời, kiểu kết quả công khai, tải model và xác minh checksum có owner riêng.

The install and verification test modules live beside their respective behavior owners. The download owner keeps its existing focused tests. This refactor does not alter source selection, paths, checksums, errors, or runtime behavior.

Test cài đặt và xác minh nằm cạnh owner xử lý tương ứng; owner download giữ các test chuyên biệt hiện có. Refactor này không đổi cách chọn nguồn, đường dẫn, checksum, lỗi hay hành vi runtime.
