# Kotlin app owner

## Responsibility
Writes Gradle settings and Kotlin entrypoint. The parent dispatcher selects this owner only for the explicit kotlin key.

## Phạm vi
Ghi Gradle settings và entrypoint Kotlin. Dispatcher cha chỉ chọn owner này khi key kotlin được yêu cầu tường minh.

## Boundary
This owner does not resolve templates, install dependencies or execute external tools.

## Ranh giới
Owner này không resolve template, cài dependency hoặc chạy công cụ bên ngoài.
