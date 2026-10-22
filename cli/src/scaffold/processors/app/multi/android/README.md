# Android output owner

## Responsibility
Android Gradle modules and entrypoint depending on shared. The coordinator invokes it in the preserved order.

## Phạm vi
Các module Gradle Android và entrypoint phụ thuộc phần shared. Bộ điều phối gọi owner theo đúng thứ tự đã giữ.

## Boundary
This owner writes its own output group and never launches framework commands.

## Ranh giới
Owner này chỉ ghi nhóm output của mình và không chạy lệnh framework.
