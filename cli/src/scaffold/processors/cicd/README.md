# CI/CD scaffold owners

## Responsibility
This folder owns the built-in CI/CD fallback after optional layer selection. CicdProcessor keeps the public support check and routes each provider to a focused output owner.

## Phạm vi
Folder này quản lý fallback CI/CD tích hợp sau khi chọn layer tùy chọn. CicdProcessor giữ kiểm tra hỗ trợ public và route từng provider tới owner output riêng.

## Preserved behavior
Unknown direct framework keys still reach the GitHub Actions writer through the existing wildcard. The create command checks the support list first; S403 preserves this behavior for a separate fail-closed fix.

## Hành vi được giữ
Framework key lạ gọi thẳng vẫn tới writer GitHub Actions qua wildcard hiện có. Create command kiểm tra support list trước; S403 giữ hành vi này cho batch fail-closed riêng.
