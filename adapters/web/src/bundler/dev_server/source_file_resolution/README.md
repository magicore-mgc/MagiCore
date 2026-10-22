# Source file resolution / Resolve file source

Owns lexical visibility policy, canonical project containment, public-directory fallback, fixed project-root file resolution and symlink target checks before a route reads or transpiles a file. The root HTML route uses the project-only entry point so `public/index.html` does not replace its generated fallback.

Sở hữu policy path lexical, giới hạn canonical trong project, fallback thư mục public, resolve file cố định trong root và kiểm tra target symlink trước khi route đọc hoặc transpile file. Route HTML root dùng entry point chỉ tìm trong project để `public/index.html` không thay thế fallback HTML đã sinh.

Dot-prefixed paths are private, except public/.well-known. Symlinks may resolve within the project root; targets outside it are forbidden. A file symlink must keep the requested extension so MIME and transpile dispatch do not silently change. Canonicalization and later path-based opens are separate operations; concurrent project-tree mutation remains a documented TOCTOU residual for HTML, CSS, static reads and the compiler.

Path bắt đầu bằng dấu chấm là riêng tư, trừ public/.well-known. Symlink được resolve trong project root; target ngoài root bị chặn. Symlink file phải giữ extension được request để MIME và dispatch transpile không đổi âm thầm. Canonicalize và thao tác mở lại bằng path là hai bước riêng; race khi project tree bị sửa đồng thời vẫn là residual TOCTOU đã ghi rõ với HTML, CSS, static và compiler.
