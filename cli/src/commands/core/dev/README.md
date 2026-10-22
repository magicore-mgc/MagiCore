# Core dev commands / Lệnh dev theo core

## Responsibility / Trách nhiệm

Routes the existing AI, App, CICD, Cloud, IoT and Web dev lanes. Public module paths stay stable; implementation belongs to each named core owner.

## Tests / Kiểm thử

Tests are under each behavior owner. AI and Web are thin delegates and keep behavior tests at their shared implementation owners.

## Tiếng Việt

Thư mục này giữ các route `mgc dev` hiện có cho AI, App, CICD, Cloud, IoT và Web. API module công khai được giữ ổn định; phần thực thi nằm trong owner theo từng core. Test đặt cạnh behavior; AI và Web chỉ ủy quyền sang implementation dùng chung.
