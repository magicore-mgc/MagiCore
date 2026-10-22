# Shared `mgc dev` command

This module validates shared flags, loads `ProjectContext`, logs the selected project, and dispatches to the existing core-owned development flow. Framework command construction and process execution belong to their core modules.

Module này kiểm tra flag dùng chung, nạp `ProjectContext`, ghi log project được chọn và chuyển lệnh sang luồng dev thuộc core. Việc tạo command framework và chạy process thuộc module core tương ứng.
