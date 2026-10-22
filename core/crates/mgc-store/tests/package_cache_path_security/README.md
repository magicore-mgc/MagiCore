# Package cache path security target / Target bảo mật path package cache

This directory owns the `package_cache_path_security.rs` Cargo integration target. It covers path construction at the public PackageCache boundary, with each test in a nearest same-name behavior folder.

Thư mục này sở hữu Cargo integration target `package_cache_path_security.rs`. Target kiểm tra tạo path qua public PackageCache; mỗi test nằm trong behavior folder cùng tên gần nhất.

The target covers metadata dot-segment containment, escape-marker separation, collision-resistant names across separators and OS-sensitive values, malformed prerelease path containment, structured version identity, ordinary-name compatibility, and recursive tarball counting. It does not exercise an attacker-preplanted symlink in a cache parent or prove cross-ecosystem cache identity.

Target kiểm tra containment metadata dot-segment, tách marker escape, chống va chạm tên qua separator và giá trị nhạy với OS, chặn path từ prerelease lỗi, identity version có cấu trúc, tương thích tên thường và đếm tarball đệ quy. Target chưa kiểm tra symlink được đặt sẵn ở cache parent và chưa chứng minh cache identity xuyên ecosystem.
