# Web package update owner

Updates the requested Web dependencies through the selected native or language backend.

## Giới hạn

This folder owns the `update` command implementation and its local wiring. It does not own registry resolution, the shared mutation service, or command-line parsing. Keep public CLI signatures stable at the Web facade. Tests remain in their current location during the source-only structural pass and will be colocated in the later test-ownership pass.

## Trách nhiệm

Cập nhật dependency Web yêu cầu qua backend native hoặc backend ngôn ngữ được chọn.

Folder này sở hữu implementation và wiring cục bộ của lệnh `update`. Folder không sở hữu registry resolver, shared mutation service hoặc CLI parsing. Giữ ổn định chữ ký công khai tại Web facade. Trong lượt refactor cấu trúc source, test giữ nguyên vị trí; sẽ chuyển gần owner ở lượt bố trí test sau.
