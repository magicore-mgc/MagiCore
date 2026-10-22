# Package source

Reads `Package.swift` through the shared regular-text guard and accepts only literal `Package(...)` / `.package(...)` forms. It does not invoke SwiftPM or evaluate source.

Đọc `Package.swift` qua regular-text guard dùng chung và chỉ nhận literal `Package(...)` / `.package(...)`. Không gọi SwiftPM hoặc thực thi source.

Unsupported syntax maps to the existing App `MgError::Unsupported` diagnostic. Tests cover literal input and computed dependency rejection.

Cú pháp chưa hỗ trợ ánh xạ về diagnostic `MgError::Unsupported` hiện có của App. Test bao phủ input literal và từ chối dependency tính toán.
