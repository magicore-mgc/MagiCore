# Offline source flags

Defines `--from-local <PATH>` and `--no-local` for scaffold commands. They select where framework source comes from when the registry layer cannot be reached; they never enable a built-in generator, because MagiCore does not embed framework source.

- `--from-local <PATH>` reuses the named project as the source, filtered copy, identity reset.
- `--no-local` refuses the local-reuse branch; the scaffold then requires the registry layer and otherwise fails closed.
- With neither flag, MagiCore looks for a matching project already scaffolded on this machine.

## Tiếng Việt

Khai báo `--from-local <PATH>` và `--no-local` cho các lệnh scaffold. Hai cờ chọn nguồn source framework khi không truy cập được registry layer; không bao giờ bật generator nội bộ, vì MagiCore không nhúng source framework.

- `--from-local <PATH>` dùng project chỉ định làm nguồn, copy có lọc, reset identity.
- `--no-local` từ chối nhánh local reuse; scaffold khi đó cần registry layer, không có thì fail-closed.
- Không truyền cờ nào: MagiCore tìm project cùng framework đã scaffold trên máy.
