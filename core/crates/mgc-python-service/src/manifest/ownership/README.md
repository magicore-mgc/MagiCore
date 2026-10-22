# Python manifest ownership

`ownership.rs` decides whether MGC can safely claim a project and whether its dependency groups fit the current native lane. It rejects foreign lockfiles, foreign dependency sources, dynamic project dependencies, unsupported PEP 508 semantics, and dependency groups other than simple PEP 735 `dev` entries.

The policy is intentionally conservative: a false result leaves the project to its existing package manager. Parsing requirements is delegated to the shared `dependency` owner so lane detection and actual manifest reads use the same grammar.

## Trách nhiệm

`ownership.rs` quyết định MGC có thể nhận quản lý project hay không, đồng thời kiểm tra các nhóm dependency có nằm trong lane native hiện tại không. Policy từ chối lockfile/nguồn dependency của tool khác, dependency project động, cú pháp PEP 508 không mô hình hóa, và dependency group ngoài nhóm `dev` dạng chuỗi đơn giản.

Policy được giữ thận trọng: trả về `false` nghĩa là để package manager hiện có tiếp tục sở hữu project. Việc phân tích requirement giao cho owner `dependency` để nhận diện lane và đọc manifest dùng cùng một grammar.
