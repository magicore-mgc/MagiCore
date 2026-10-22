# iOS native development lane

## Responsibility / Trách nhiệm

Owns Swift/Objective-C iOS lane selection, macOS/Xcode guards, simulator arguments and dry-run behavior.

## Tests / Kiểm thử

`cli/tests/app_empty_swift_dev_e2e.rs` exercises the SwiftPM CLI lane. Objective-C/Xcode simulator execution still needs a macOS/Xcode E2E lane.

## Tiếng Việt

Owner này chọn lane Swift/Objective-C iOS, kiểm tra điều kiện macOS/Xcode, tạo tham số simulator và xử lý dry-run. `cli/tests/app_empty_swift_dev_e2e.rs` bao phủ lane SwiftPM; thực thi Objective-C/Xcode Simulator vẫn cần E2E trên macOS có Xcode.
