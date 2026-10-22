# Web workspace layout owners

This is the single source of truth for Web workspace configuration, project mode, frontend/backend roots, dev target selection, and package install target order. Dev and package lifecycle commands consume these owners so the same project cannot be classified differently by two Web commands.

`manifest_presence/` owns the shared backend-marker predicate; `types/` owns the serde models; `project_mode/` resolves mode and conventional app directories; `dev_target/` selects the dev root; `install_targets/` expands project mode to ordered install roots. Detection failures and fallback behavior stay in the owner that makes the decision.

# Owner workspace layout Web

Đây là nguồn chuẩn duy nhất cho cấu hình workspace Web, project mode, root frontend/backend, chọn dev target và thứ tự install target. Lệnh dev và package lifecycle cùng dùng các owner này để một project không bị phân loại khác nhau giữa hai lệnh.

`manifest_presence/` sở hữu predicate backend-marker dùng chung; `types/` sở hữu model serde; `project_mode/` resolve mode và thư mục app chuẩn; `dev_target/` chọn root dev; `install_targets/` mở rộng mode thành danh sách root cài có thứ tự. Lỗi nhận diện và fallback thuộc owner đưa ra quyết định.
