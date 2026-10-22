# iOS and Objective-C bridge output owner

## Responsibility
Swift Package executable and Objective-C bridge sources. The coordinator invokes it in the preserved order.

## Phạm vi
Executable Swift Package và source cầu nối Objective-C. Bộ điều phối gọi owner theo đúng thứ tự đã giữ.

## Boundary
This owner writes its own output group and never launches framework commands.

## Ranh giới
Owner này chỉ ghi nhóm output của mình và không chạy lệnh framework.
