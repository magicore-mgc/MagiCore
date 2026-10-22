# Multi-platform App scaffold

## Responsibility
Preserves the existing output sequence: KMP shared, Android, iOS/Objective-C, React Native, then Flutter.

## Phạm vi
Giữ thứ tự output hiện có: KMP shared, Android, iOS/Objective-C, React Native, rồi Flutter.

## Known behavior
Some existing Kotlin generated strings contain literal {pkg}. This structural refactor preserves those bytes; a separate behavior fix should replace them and add direct output coverage.

## Hành vi đã biết
Một số chuỗi Kotlin được sinh hiện chứa nguyên văn {pkg}. Refactor cấu trúc giữ nguyên byte này; batch sửa hành vi riêng cần thay chúng và bổ sung test output trực tiếp.
