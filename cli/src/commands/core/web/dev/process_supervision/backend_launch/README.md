# Web dev backend launch owner

Turns the resolved backend start command into a `DevLaunch`. A Node starter keeps `build_dev_launch` (flag policy, optimizer environment, audit log, hooks); every other runner launches its own mature tool with the shared `HOST`/`PORT`/`SERVER_PORT` contract exported from the centralized `dev_port` table. A `server/` directory with no startable manifest is reported with the manifests that would fix it, never spawned as an empty process.

Chuyển lệnh start backend đã resolve thành `DevLaunch`. Starter Node giữ `build_dev_launch` (policy flag, env optimizer, audit log, hook); mọi runner khác chạy công cụ chín của nó với contract `HOST`/`PORT`/`SERVER_PORT` xuất từ bảng `dev_port` trung tâm. Thư mục `server/` không có manifest khởi động được sẽ được báo kèm manifest cần thêm, không bao giờ spawn process rỗng.
