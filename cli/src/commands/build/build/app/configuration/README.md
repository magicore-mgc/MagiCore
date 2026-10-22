# App build configuration / Cấu hình build App

## Responsibility / Trách nhiệm

Safely reads `mgc.toml`, validates `[app].language`, and infers a runtime only where the existing contract permits. It does not modify configuration or invent a default for malformed explicit input.

Đọc an toàn `mgc.toml`, kiểm tra `[app].language` và chỉ suy luận runtime khi contract hiện tại cho phép. Không sửa cấu hình hoặc tự đặt mặc định khi input tường minh bị lỗi.

## Errors and tests / Lỗi và test

Read, parse, invalid-language and ambiguous-language failures propagate with existing diagnostics. `AppConfigurationError` now owns read/parse/language/ambiguity variants and keeps parser/filesystem sources. Existing unit tests are still registered in the CLI test module.

Lỗi đọc/parse/ngôn ngữ sai/mơ hồ được propagate với diagnostic hiện tại. `AppConfigurationError` sở hữu lỗi đọc/parse/language/mơ hồ và giữ source parser/filesystem. Unit test vẫn đăng ký ở module test CLI.
