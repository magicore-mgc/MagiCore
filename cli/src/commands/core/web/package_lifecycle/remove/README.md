# Web package remove owner

Removes declared Web dependencies while preserving the native/Lib backend routing and ownership gate.

## Giới hạn

This folder owns the `remove` command implementation and its local wiring. It does not own registry resolution, the shared mutation service, or command-line parsing. Keep public CLI signatures stable at the Web facade. Tests remain in their current location during the source-only structural pass and will be colocated in the later test-ownership pass.

## Trách nhiệm

Gỡ dependency Web, giữ nguyên định tuyến backend native/Lib và ownership gate.

Folder này sở hữu implementation và wiring cục bộ của lệnh `remove`. Folder không sở hữu registry resolver, shared mutation service hoặc CLI parsing. Giữ ổn định chữ ký công khai tại Web facade. Trong lượt refactor cấu trúc source, test giữ nguyên vị trí; sẽ chuyển gần owner ở lượt bố trí test sau.
