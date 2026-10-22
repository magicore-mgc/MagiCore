# Source inventory / Kiểm kê source

`inventory.py` owns deterministic Rust, Python and shell file enumeration and per-file read adapters. Read failures keep the existing fail-closed finding behavior; no scan root is silently dropped.

`inventory.py` sở hữu việc liệt kê Rust, Python, shell theo thứ tự ổn định và adapter đọc từng file. Lỗi đọc giữ nguyên finding fail-closed; không âm thầm bỏ root.
