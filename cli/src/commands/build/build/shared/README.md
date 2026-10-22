# Shared build policy / Policy build dùng chung

## Responsibility / Trách nhiệm

This module composes three independent policies: project discovery/manifest checks, PATH/PATHEXT tool lookup and Go's no-network environment. Keep these policies shared; do not let a core silently install dependencies or redirect MGC-owned caches.

Module này ghép ba policy độc lập: tìm project/kiểm tra manifest, dò tool theo PATH/PATHEXT và môi trường Go không mạng. Dùng chung các policy này; không để core tự cài dependency hoặc chuyển hướng cache do MGC sở hữu.

## Errors / Lỗi

Project I/O errors propagate; tool lookup and environment construction are infallible. There is no local error type because these operations do not define a distinct diagnostic domain. Existing callers and tests remain unchanged in this batch.

Lỗi I/O project được propagate; dò tool và dựng environment không phát sinh lỗi. Không tạo error type cục bộ vì các thao tác này chưa có taxonomy diagnostic riêng. Caller và test hiện tại được giữ nguyên trong batch này.
