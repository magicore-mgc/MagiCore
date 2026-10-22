# Doctor errors

The presenter owns JSON serialization failures and preserves the original `serde_json::Error` source. Do not map serialization failure to a successful or partial report.

# Lỗi doctor

Presenter sở hữu lỗi serialization JSON và giữ nguyên `serde_json::Error` nguồn. Không chuyển lỗi serialize thành report thành công hoặc một phần.
