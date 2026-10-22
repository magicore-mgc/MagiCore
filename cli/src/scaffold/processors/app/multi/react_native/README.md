# React Native output owner

## Responsibility
Package metadata and JavaScript application entrypoints. The coordinator invokes it in the preserved order.

## Phạm vi
Metadata package và entrypoint ứng dụng JavaScript. Bộ điều phối gọi owner theo đúng thứ tự đã giữ.

## Boundary
This owner writes its own output group and never launches framework commands.

## Ranh giới
Owner này chỉ ghi nhóm output của mình và không chạy lệnh framework.
