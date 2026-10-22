# Web package list owner

Reads the Web package inventory using the owning adapter and records the Web ownership decision.

## Giới hạn

This folder owns the `list` command implementation and its local wiring. It does not own registry resolution, the shared mutation service, or command-line parsing. Keep public CLI signatures stable at the Web facade. Tests remain in their current location during the source-only structural pass and will be colocated in the later test-ownership pass.

## Trách nhiệm

Đọc danh sách package Web bằng adapter sở hữu và ghi nhận quyết định ownership.

Folder này sở hữu implementation và wiring cục bộ của lệnh `list`. Folder không sở hữu registry resolver, shared mutation service hoặc CLI parsing. Giữ ổn định chữ ký công khai tại Web facade. Trong lượt refactor cấu trúc source, test giữ nguyên vị trí; sẽ chuyển gần owner ở lượt bố trí test sau.
