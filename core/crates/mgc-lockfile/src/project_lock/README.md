# Project writer lock / Khóa writer theo project

This folder preserves `mgc_lockfile::project_lock::*` and separates kernel-lock acquisition, contention classification, and path/reparse checks. `mod.rs` contains wiring and explicit exports only.

Thư mục này giữ `mgc_lockfile::project_lock::*` và tách acquire khóa kernel, phân loại contention, cùng kiểm tra symlink/reparse. `mod.rs` chỉ nối module và export tường minh.

The guard remains RAII and timeout never reclaims a live lock. Current pathname check-then-open behavior is preserved; S291 records its parent-swap TOCTOU limitation and does not claim to harden it.

Guard tiếp tục dùng RAII, timeout không cướp lock còn sống. Hành vi check-then-open theo pathname hiện tại được giữ; S291 ghi rõ giới hạn TOCTOU khi swap parent và không tuyên bố đã harden.
