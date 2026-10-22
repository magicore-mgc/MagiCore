# Flutter output owner

## Responsibility
Package metadata and Dart application entrypoint. The coordinator invokes it in the preserved order.

## Phạm vi
Metadata package và entrypoint ứng dụng Dart. Bộ điều phối gọi owner theo đúng thứ tự đã giữ.

## Boundary
This owner writes its own output group and never launches framework commands.

## Ranh giới
Owner này chỉ ghi nhóm output của mình và không chạy lệnh framework.
