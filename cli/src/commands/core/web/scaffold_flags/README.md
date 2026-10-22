# Web scaffold flags

This owner defines Web-only scaffold options such as project layout, frontend/backend frameworks, styling, state management, APIs, testing, and Web deployment integrations. Its Docker and Kubernetes options remain here because Web has a distinct Kubernetes action and help text (`--k8s` creates a local development manifest and enables Docker). The shared non-Web `ContainerFlags` owner serves AI, Cloud, App, and Lib create commands; keeping separate DTOs avoids implying identical behavior.

The DTO does not perform validation or filesystem writes. Web parser and scaffold behavior tests stay with the command-definition and Web test owners; this owner has no separate fallible error type.

## Tiếng Việt

Owner này khai báo các cờ scaffold riêng của Web như kiểu project, framework frontend/backend, CSS, state management, API, test và tích hợp triển khai Web. Cờ Docker/Kubernetes của Web ở lại đây vì tác vụ và trợ giúp `--k8s` riêng (tạo manifest phát triển local và bật Docker). Owner `ContainerFlags` dùng chung cho lệnh create AI, Cloud, App, Lib; tách DTO giúp không tạo ấn tượng các core có hành vi giống nhau.

DTO không tự validate hay ghi filesystem. Test parser và scaffold Web tiếp tục nằm ở owner định nghĩa lệnh và test Web; module này không có xử lý lỗi có thể thất bại riêng.
