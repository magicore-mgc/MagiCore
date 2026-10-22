# KMP shared output owner

## Responsibility
KMP common source and multiplatform Gradle configuration. The coordinator invokes it in the preserved order.

## Phạm vi
Source common KMP và cấu hình Gradle đa nền tảng. Bộ điều phối gọi owner theo đúng thứ tự đã giữ.

## Boundary
This owner writes its own output group and never launches framework commands.

## Ranh giới
Owner này chỉ ghi nhóm output của mình và không chạy lệnh framework.
