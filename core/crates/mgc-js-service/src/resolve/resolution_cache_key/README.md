# Resolution cache-key owner / Owner khóa cache resolve

This owner builds the stable SHA-256 identity from sorted dependency-group entries and the normalized registry URL. The cache-key version prefix and input encoding are compatibility contracts.

Owner này tạo identity SHA-256 ổn định từ dependency-group đã sắp xếp và registry URL đã normalize. Prefix version cache key và encoding input là hợp đồng tương thích.

Nearest tests cover dependency-order/app-name independence, registry trailing-slash normalization, and registry/range sensitivity. Cache file IO and TTL remain outside this owner.

Test gần nhất kiểm tra không phụ thuộc thứ tự dependency/tên app, normalize slash cuối registry và nhạy với registry/range. File IO cache và TTL nằm ngoài owner.
