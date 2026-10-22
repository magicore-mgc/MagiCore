# Store Prune Tests

These nearest-owner tests exercise the real project-local SQLite store and filesystem path for missing-DB no-op, dry-run, deletion, Web install lock contention and symlink refusal. A separate report contract test pins the existing serialized field names and types.

Các test gần owner chạy trên SQLite store và path filesystem project-local thật để kiểm tra DB thiếu, dry-run, xóa, tranh chấp lock Web install và từ chối symlink. Test report riêng giữ nguyên tên và kiểu field đã serialize.

The cases are authored for later execution on supported Unix and Windows lanes; they have not been run in the source-first phase.

Các case được viết để chạy sau trên lane Unix và Windows được hỗ trợ; chưa chạy trong phase refactor source.
