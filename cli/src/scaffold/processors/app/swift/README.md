# Swift Package app owner

## Responsibility
Writes Manifest, executable source and XCTest target. The parent dispatcher selects this owner only for the explicit swift key.

## Phạm vi
Ghi Manifest, source executable và target XCTest. Dispatcher cha chỉ chọn owner này khi key swift được yêu cầu tường minh.

## Boundary
This owner does not resolve templates, install dependencies or execute external tools.

## Ranh giới
Owner này không resolve template, cài dependency hoặc chạy công cụ bên ngoài.
