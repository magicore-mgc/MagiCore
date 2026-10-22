# Shared server types / Type dùng chung của server

Owns the public project configuration and the shared private request state consumed by all route owners.

Sở hữu cấu hình project public và state request private dùng chung cho các owner route.

There is no independent error contract here; I/O and HTTP error mapping stay with their operation owners.

Không có error contract độc lập; lỗi I/O và HTTP được ánh xạ tại owner thao tác tương ứng.
