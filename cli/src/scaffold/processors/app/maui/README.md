# MAUI app owner

## Responsibility
Writes MAUI project and XAML application/page entrypoints. The parent dispatcher selects this owner only for the explicit maui key.

## Phạm vi
Ghi Project MAUI và entrypoint ứng dụng/trang XAML. Dispatcher cha chỉ chọn owner này khi key maui được yêu cầu tường minh.

## Boundary
This owner does not resolve templates, install dependencies or execute external tools.

## Ranh giới
Owner này không resolve template, cài dependency hoặc chạy công cụ bên ngoài.
