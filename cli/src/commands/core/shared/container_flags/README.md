# Shared container flags

This owner defines the shared `--docker` and `--k8s` scaffold arguments used by the currently wired AI, Cloud, App, and Lib create commands. Web keeps its own flags because its `--k8s` behavior and help text describe a local development manifest. The DTO only describes arguments; each core command retains responsibility for accepting, rejecting, or applying a selected option.

The Web options are not flattened into this type because that would alter their help contract and suggest behavior equivalence. Parser and per-core dispatch tests remain with their existing command owners. No runtime capability is implied for a core that does not currently declare these flags.

## Tiếng Việt

Owner này khai báo hai tham số scaffold dùng chung `--docker` và `--k8s` cho các lệnh create AI, Cloud, App, Lib hiện đang nối. Web có owner riêng vì hành vi và nội dung trợ giúp `--k8s` mô tả manifest phát triển local.

Không flatten DTO Web vào type này vì sẽ làm đổi contract trợ giúp và ngụ ý các core có hành vi tương đương. Test parser và dispatch vẫn ở owner lệnh hiện tại. Không suy ra core chưa khai báo cờ đã có capability runtime.
