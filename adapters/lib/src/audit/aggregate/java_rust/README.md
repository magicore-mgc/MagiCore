# Java + Rust audit / Audit Java + Rust

Builds the shared plan with the Java lane before Rust. If `java_step` declines (for example, a Maven project without the constructor's Gradle gate), it retains the explicit OSV Maven `ScanStep` so an honest Unsupported report remains visible.

Tạo plan dùng chung với lane Java trước Rust. Nếu `java_step` từ chối (ví dụ project Maven không qua gate Gradle của constructor), owner giữ `ScanStep` OSV Maven tường minh để report Unsupported trung thực vẫn xuất hiện.

This fallback is security-report behavior, not dead compatibility code. Preserve its order and metadata; direct tests are pending and were not run here.

Fallback này ảnh hưởng report bảo mật, không phải compatibility code thừa. Giữ nguyên thứ tự/metadata; test trực tiếp còn pending và chưa chạy ở đây.
