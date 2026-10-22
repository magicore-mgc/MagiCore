# Lib offline install policy / Policy install offline Lib

EN: Owns the preflight allowlist for offline Lib installs. TypeScript is allowed to reach the Web delegate; Rust/Python retain their verified cache-only lanes; Go/Java/.NET keep the current fail-closed error. The check runs before lock reads or writes.

VI: Sở hữu allowlist preflight cho install Lib offline. TypeScript được chuyển tiếp sang Web delegate; Rust/Python giữ lane cache-only đã xác minh; Go/Java/.NET giữ lỗi fail-closed hiện tại. Gate chạy trước khi đọc hoặc ghi lock.
