# Workspace discovery / Khám phá workspace

## English

This owner coordinates configured workspace target discovery and exposes the existing workspace graph and layout helpers through the crate facade. Configuration parsing, path safety, recursive package scanning, and graph construction have separate owners. It intentionally preserves the existing public crate-root routes and does not choose a new package-manager policy.

S227 source-static extraction has completed both review loops; executable checks remain deferred until the repository-wide source pass is complete and machine-used storage is below the user's limit. See ProgressReport.md and Checklist.md for the current local-only record.

## Tiếng Việt

Owner này điều phối việc tìm package theo layout workspace và giữ các helper layout/graph hiện có qua facade crate. Parser cấu hình, an toàn đường dẫn, quét package đệ quy và dựng graph có owner riêng. Nó giữ nguyên đường API hiện tại, không tự đặt chính sách package-manager mới.

Việc tách source S227 đã qua hai vòng rà tĩnh; kiểm tra thực thi vẫn hoãn đến khi xong lượt refactor source toàn repo và dung lượng máy dưới giới hạn người dùng đặt. Xem ProgressReport.md và Checklist.md để biết trạng thái local.
