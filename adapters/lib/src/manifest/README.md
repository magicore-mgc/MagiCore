# Library manifest owners / Owner manifest thư viện

**Purpose / Mục đích:** Keep language-specific library manifest parsing and mutation under small owners, with this module providing only the stable crate::manifest wiring. Đặt parser và mutation manifest thư viện theo từng ngôn ngữ; module này chỉ giữ wiring ổn định cho crate::manifest.

**Errors / Lỗi:** Owners retain MgResult and existing fail-closed MgError branches; no duplicate local error enum is introduced. Owner giữ MgResult và các nhánh fail-closed MgError hiện có; không tạo enum lỗi cục bộ trùng lặp.

**Tests / Kiểm thử:** Nearest manifest tests are registered from `test/mod.rs` and grouped by format/parser/writer responsibility in `test/`; see that folder's README and progress report. Test manifest gần owner được đăng ký từ `test/mod.rs` và chia theo trách nhiệm format/parser/writer trong `test/`; xem README và progress report tại folder đó.
