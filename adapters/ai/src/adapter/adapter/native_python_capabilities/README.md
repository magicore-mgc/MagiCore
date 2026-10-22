# Native Python capabilities

## Responsibility

This owner wires four private `AiAdapter` trait implementations: dependency resolution, lock/manifest writes, artifact fetching and content-store installation. Each trait has its own same-name module folder and bilingual README. Unsupported lanes continue to use shared `unsupported_capability`; ownership checks and diagnostics remain in the lockfile owner. No module-local error type is needed because these implementations add no new error category.

## Trách nhiệm

Owner này nối bốn implementation trait private của `AiAdapter`: resolve dependency, ghi lock/manifest, fetch artifact và cài content store. Mỗi trait có folder module cùng tên và README song ngữ. Lane không hỗ trợ vẫn dùng `unsupported_capability` chung; guard ownership và diagnostic nằm trong owner lockfile. Không cần error type riêng vì các implementation này không thêm error category mới.
