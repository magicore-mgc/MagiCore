# Shared CLI project context / Context project dùng chung của CLI

This owner loads the project root and config, chooses the core adapter, and exposes the project execution profile to shared CLI commands. `mod.rs` only wires the resolver and public `ProjectContext` export; `context.rs` owns lifecycle and adapter selection.

Owner này nạp root và config project, chọn adapter theo core, rồi cung cấp execution profile cho các lệnh CLI dùng chung. `mod.rs` chỉ nối resolver và export `ProjectContext`; `context.rs` sở hữu lifecycle cùng lựa chọn adapter.

Core identity/config branch rules live under `config_resolution/`; owner progress and its local checklist live under `docs/`. Public paths through both the CLI library and binary remain `context::ProjectContext`. Context accessor test source is in `test/`; runtime execution is pending the repository-wide source closure and storage gate.

Quy tắc identity/config của từng nhánh nằm trong `config_resolution/`; tiến độ và checklist cục bộ của owner nằm trong `docs/`. Giữ nguyên đường dẫn public `context::ProjectContext` cho cả thư viện CLI và binary. Test accessor nằm tại `test/`; chưa chạy runtime test cho đến khi đóng refactor source toàn repo và đạt giới hạn dung lượng.
