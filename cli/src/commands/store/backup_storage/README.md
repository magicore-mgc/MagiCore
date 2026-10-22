# Backup storage owners / Owner lưu trữ backup

This module wires independent owners for tree copy, safe clearing, and default backup-path resolution. Tree copying validates the complete source before destination creation, streams regular files, and makes a second link pass without keeping one in-memory record per link. Destination roots are created componentwise and checked without following the final component. Each operation has a local error type, nearest tests, and a bilingual README. `mod.rs` only declares and re-exports owners.

Module này nối các owner riêng cho copy cây, clear an toàn và phân giải path backup mặc định. Copy cây xác thực toàn bộ source trước khi tạo đích, stream file thường và quét link ở lượt hai mà không giữ một record trong memory cho mỗi link. Đích copy phải chưa có hoặc đang trống; root đích được tạo từng component và kiểm tra không follow component cuối. Mỗi thao tác có error type, test gần nhất và README song ngữ. `mod.rs` chỉ khai báo và export owner.
