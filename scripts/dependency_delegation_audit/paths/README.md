# Audit paths / Đường dẫn audit

`paths.py` resolves repository, scripts and ledger paths from the package location. Existing ledger path spelling is retained when passed through the old CLI. Invalid checkout layout remains a visible path/read failure.

`paths.py` xác định repo, scripts và ledger path từ vị trí package. Giữ path ledger tương thích với CLI cũ; checkout sai cấu trúc phải lộ lỗi đường dẫn/đọc.
