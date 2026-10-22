# Optimizer adapters

## Purpose

Each adapter owns the runtime predicate and optimization settings for one runtime family. Shared runtime identity comes from `optimizer::runtime_detect`; this folder does not inspect project manifests or dispatch CLI commands.

## Phạm vi

Mỗi adapter sở hữu điều kiện khớp runtime và cấu hình tối ưu cho một nhóm runtime. Danh tính runtime dùng chung lấy từ `optimizer::runtime_detect`; folder này không tự đọc manifest project hay điều phối lệnh CLI.
