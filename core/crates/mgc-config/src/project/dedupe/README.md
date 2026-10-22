# Project dedupe settings owner

This owner contains the generic `[dedupe]` configuration DTO. The project facade keeps `ProjectConfig::dedupe` and re-exports `DedupeConfig` at `mgc_config::project::DedupeConfig`. Missing `prefer` remains `false`; dedupe execution policy is owned by the consumer and is not implemented here.

The DTO is infallible and contains no validation or recovery behavior, so this folder does not define an `error.rs`. Nearest serde tests pin the default and explicit boolean representation; project-level TOML coverage remains in the crate integration tests.

# Owner cấu hình dedupe dùng chung của project

Owner này chứa DTO cấu hình generic `[dedupe]`. Facade project vẫn sở hữu `ProjectConfig::dedupe` và re-export `DedupeConfig` qua `mgc_config::project::DedupeConfig`. Khi thiếu `prefer`, giá trị vẫn là `false`; policy thực thi dedupe thuộc consumer và không được triển khai ở đây.

DTO không có nhánh validate hay recovery, nên folder không định nghĩa `error.rs`. Test serde gần owner khóa default và biểu diễn boolean tường minh; test TOML cấp project vẫn nằm trong integration tests của crate.
