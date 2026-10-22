# Search cache owner / Owner cache tìm kiếm

This folder owns the stable SearchCache API and local SQLite search metadata. The current production caller uses learned registry choices; result get/insert APIs are not wired into the package-search flow.
Thư mục này sở hữu API SearchCache ổn định và metadata tìm kiếm trong SQLite cục bộ. Caller production hiện dùng lựa chọn registry đã học; API get/insert kết quả chưa nối vào luồng tìm package.

The facade lives in cache.rs. Schema setup, result rows, learned registry choices, and local errors have separate owners. The crate root continues to export the same public SearchCache type.
Facade nằm trong cache/cache.rs. Schema, cache kết quả, registry user đã chọn và lỗi cục bộ có owner riêng. Crate root tiếp tục export cùng kiểu SearchCache.

The database stays at MGC_DATA_DIR/search_cache.db. Existing query keys, seven-day TTL, SQL ordering, and error boundary are preserved by this structural slice. The production search flow currently does not call result-cache get/insert, so result reuse is not an active optimization. Expired rows are not cleaned and the database has no size limit; retention/GC is a separate unresolved behavior change. Opening the DB currently propagates an error before online search, and choice-write errors propagate after selection; fallback behavior is also unresolved.
Database vẫn ở MGC_DATA_DIR/search_cache.db. Lát cắt cấu trúc này giữ nguyên query key, TTL bảy ngày, thứ tự SQL và error boundary. Bản ghi hết hạn chưa được dọn và database chưa có giới hạn dung lượng; retention/GC là thay đổi hành vi riêng còn mở. Lỗi mở DB hiện propagate trước online search; lỗi ghi lựa chọn propagate sau khi chọn package, chưa có fallback.

Nearest-owner source tests exist under each populated owner test directory and are NOT_RUN. Existing integration tests remain at core/crates/mgc-search/tests/cache.rs and are frozen.
Test source gần từng owner nằm trong thư mục test tương ứng và chưa chạy. Integration tests hiện tại vẫn ở core/crates/mgc-search/tests/cache.rs và được giữ nguyên.
