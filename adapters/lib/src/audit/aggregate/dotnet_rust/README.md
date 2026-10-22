# .NET + Rust audit / Audit .NET + Rust

Builds the shared plan with the .NET lane before Rust. If `dotnet_step` declines (for example, no root `.csproj`), it retains the explicit OSV NuGet `ScanStep` so an honest Unsupported report remains visible.

Tạo plan dùng chung với lane .NET trước Rust. Nếu `dotnet_step` từ chối (ví dụ không có `.csproj` ở root), owner giữ `ScanStep` OSV NuGet tường minh để report Unsupported trung thực vẫn xuất hiện.

This fallback is security-report behavior. Preserve its order and metadata; direct tests are pending and were not run here.

Fallback này ảnh hưởng report bảo mật. Giữ nguyên thứ tự/metadata; test trực tiếp còn pending và chưa chạy ở đây.
