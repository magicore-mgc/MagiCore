# Core package dispatch / Điều phối package theo core

This module routes add/remove/list/update commands to one of nine core owners. Each core owner retains its command mapping and any feature-disabled fallback present in the old dispatcher; the existing Web route has no feature-disabled gate. The facade preserves the existing `dispatch_core -> package_ops::dispatch` route. Errors use the existing `anyhow::Result` boundary and shared `core_not_in_build` error; no new error/type contract is introduced.

Module này định tuyến lệnh add/remove/list/update tới một trong chín owner core. Mỗi core giữ mapping lệnh và fallback feature nếu dispatcher cũ có; route Web hiện tại không có feature gate. Facade bảo toàn route `dispatch_core -> package_ops::dispatch` hiện tại. Lỗi dùng boundary `anyhow::Result` sẵn có và lỗi chung `core_not_in_build`; không phát sinh contract error/type mới.

This source split does not claim CLI/runtime verification. Tests are authored and remain NOT_RUN until the whole-project source pass closes.

Việc tách source này không khẳng định đã kiểm chứng CLI/runtime. Test đã được viết và vẫn NOT_RUN đến khi source pass toàn dự án hoàn tất.
