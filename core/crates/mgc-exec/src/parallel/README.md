# Parallel execution

Owns the public parallel task API and coordinates preflight, scoped task execution, output and ordered result reporting through the shared guarded executor.

## Scope

The current Rust source map finds no production caller; the public API is exercised by tests. Do not assume a CLI/runtime route exists without a caller.

# Thực thi song song — tiếng Việt

Sở hữu API task song song công khai và điều phối preflight, chạy task có scope, output và báo kết quả theo thứ tự qua executor có guard dùng chung.

## Phạm vi

Source map Rust hiện không thấy production caller; API công khai đang được test gọi. Không suy ra có luồng CLI/runtime nếu chưa tìm thấy caller.
