# Package graph errors

Owns typed failures raised while translating resolver output into installable package graph records. The current refusal means the resolver reported a direct root identity that is absent from its resolved package set; graph assembly must stop instead of guessing a root from package name or semver range.

Sở hữu lỗi typed khi chuyển output resolver thành record graph để cài. Refusal hiện tại báo resolver đưa ra root trực tiếp nhưng identity đó không có trong tập package đã resolve; assembly phải dừng thay vì đoán root theo tên package hoặc semver range.
