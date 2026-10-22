# Shared child-process policy

This owner exposes stable run-local process-policy functions and the shared ForbiddenProcess record. The root process_policy.rs coordinates and re-exports three focused responsibilities: argv/ classifies native argv and observed process environment, including the root `ExecutionScope` rule for monitored `adb`/`xcrun` descendants; command_line/ preserves the legacy whitespace-token matcher and name projection; child_command_line/ validates that a monitored child has a native argv snapshot. process_identity/ owns executable normalization and exact shell-probe recognition; invocation_shape/ renders bounded redacted diagnostics; script_guard/ checks a script shebang. error.rs remains the shared typed error boundary. Public run and downstream call paths remain unchanged.

The shebang guard intentionally preserves its current behavior: non-files, unreadable files, and files without a shebang are accepted. The legacy command-line matcher still scans whitespace-separated text. These are existing policy limits, not claims that the inputs are safe.

# Policy child process dùng chung

Owner này xuất các hàm policy ổn định trong phạm vi run và record ForbiddenProcess dùng chung. process_policy.rs ở root điều phối và export ba trách nhiệm riêng: argv/ phân loại native argv cùng environment của process được quan sát, gồm rule `ExecutionScope` của root với descendant `adb`/`xcrun`; command_line/ giữ matcher legacy theo token whitespace và wrapper lấy tên; child_command_line/ xác thực child được monitor có native argv snapshot. process_identity/ sở hữu chuẩn hóa executable và nhận diện shell probe chính xác; invocation_shape/ render diagnostic redacted có giới hạn; script_guard/ kiểm tra shebang. error.rs vẫn là boundary lỗi typed chung. Public call path run/downstream không đổi.

Guard shebang cố ý giữ behavior hiện tại: đường dẫn không phải file, file không đọc được và file không có shebang đều được chấp nhận. Matcher command-line legacy vẫn dò text tách theo whitespace. Đây là giới hạn policy hiện tại, không phải khẳng định input an toàn.
