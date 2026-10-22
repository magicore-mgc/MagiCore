# Tauri app owner

## Responsibility
Writes Frontend, Rust application and Tauri configuration. The parent dispatcher selects this owner only for the explicit tauri key.

## Phạm vi
Ghi Frontend, ứng dụng Rust và cấu hình Tauri. Dispatcher cha chỉ chọn owner này khi key tauri được yêu cầu tường minh.

## Boundary
This owner does not resolve templates, install dependencies or execute external tools.

## Ranh giới
Owner này không resolve template, cài dependency hoặc chạy công cụ bên ngoài.
