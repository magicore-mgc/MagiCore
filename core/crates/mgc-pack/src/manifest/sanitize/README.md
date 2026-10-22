# Manifest sanitizer / Bộ làm sạch manifest

Validates the required package identity, blocks `private: true` and present non-boolean `private` values, removes `private` and `publishConfig`, and preserves all other JSON while applying non-empty name/version overrides.

Kiểm tra name/version bắt buộc, chặn `private: true` và giá trị `private` sai kiểu, bỏ `private` và `publishConfig`, giữ nguyên JSON còn lại và áp dụng override name/version không rỗng.
