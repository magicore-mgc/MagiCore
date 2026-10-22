# Web package lifecycle owners

This folder contains one owner per public package lifecycle command. Each command folder has a same-name implementation file, a `mod.rs` export boundary, and a README describing its responsibility. The parent `mod.rs` is the only lifecycle wiring point; `web.rs` re-exports these APIs to keep existing command callers stable.

`add/` and `remove/` route manifest changes through the shared mutation service. `list/` reads package state. `update/` selects the owning adapter. `install/` coordinates preflight, backend routing, workspace installation, dedupe and lock updates. Registry, store, lockfile schema and CLI argument parsing remain owned by their existing subsystems.

Tests remain where they currently live during the source-only pass; test relocation and executable verification follow after the repository-wide source structure pass.

# Owner lifecycle package Web

Folder này có một owner cho mỗi lệnh lifecycle package công khai. Mỗi folder lệnh có file implementation trùng tên, ranh giới export `mod.rs` và README mô tả trách nhiệm. `mod.rs` cha là điểm nối lifecycle duy nhất; `web.rs` export lại API để giữ nguyên các caller hiện tại.

`add/` và `remove/` đổi manifest qua shared mutation service. `list/` đọc trạng thái package. `update/` chọn adapter sở hữu. `install/` điều phối preflight, định tuyến backend, cài workspace, dedupe và cập nhật lock. Registry, store, schema lockfile và phân tích đối số CLI tiếp tục thuộc các subsystem hiện có.

Trong lượt source-only, test giữ tại vị trí hiện tại; chuyển test và chạy kiểm chứng sẽ thực hiện sau khi hoàn tất cấu trúc source toàn repository.
