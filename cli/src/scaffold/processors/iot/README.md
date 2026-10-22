# IoT scaffold owners

## Responsibility
This folder owns the native IoT fallback reached by the core writer. The public IotProcessor keeps the board catalog, direct-support check and framework dispatch stable while generator owners write their output groups.

## Phạm vi
Folder này quản lý fallback IoT native được core writer gọi. IotProcessor public giữ board catalog, kiểm tra framework tự sinh và dispatch ổn định; các owner generator ghi nhóm output riêng.

## Preserved behavior
The generic Rust writer remains the current catch-all for frameworks reaching the fallback directly. create-iot currently validates supported frameworks before that route; S402 does not change either behavior.

## Hành vi được giữ
Writer Rust tổng quát vẫn là nhánh catch-all hiện tại nếu framework đi thẳng vào fallback. create-iot hiện kiểm tra framework trước route đó; S402 không đổi hai hành vi này.
