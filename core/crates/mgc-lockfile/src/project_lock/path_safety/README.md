# Project path safety / An toàn path project

This owner creates/checks the `.magicore` directory, refuses symlink/reparse guard files, and exports the shared predicate used by patch/journal writers. Its local typed errors preserve I/O sources and map to existing `WriteFailed` messages.

Owner này tạo/kiểm tra directory `.magicore`, từ chối guard symlink/reparse, và export predicate dùng chung cho patch/journal writer. Typed error cục bộ giữ source I/O và ánh xạ về message `WriteFailed` hiện tại.

The check remains pathname-based and is vulnerable to a concurrent parent swap between metadata checks and open. S291 records this as an unresolved hardening task.

Kiểm tra vẫn dựa trên pathname và có thể bị swap parent đồng thời giữa bước metadata và open. S291 ghi đây là đầu việc hardening chưa giải quyết.
