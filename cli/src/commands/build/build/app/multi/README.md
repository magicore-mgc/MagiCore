# Multi-platform App build / Build App đa nền tảng

## Responsibility / Trách nhiệm

Validates the full selected-platform list before invoking tools, attempts each supported native runner and returns an incomplete result when selected work is skipped. It does not treat skipped targets as success.

Kiểm tra toàn bộ danh sách platform được chọn trước khi gọi tool, chạy runner native được hỗ trợ và trả kết quả incomplete khi target được chọn bị bỏ qua. Không coi target bị skip là thành công.

## Errors and tests / Lỗi và test

Invalid platform configuration, missing runner and incomplete aggregate results follow current build diagnostics. `AppMultiBuildError` owns invalid platform configuration and incomplete selected-target results. Owner-local tests remain for the later test phase.

Config platform sai, thiếu runner và kết quả tổng hợp incomplete theo diagnostic build hiện tại. `AppMultiBuildError` sở hữu lỗi cấu hình platform và kết quả thiếu target đã chọn. Test tại owner để pha test sau.
