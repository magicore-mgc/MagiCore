# npm search response / Response tìm kiếm npm

Owns the single NPM search-result object graph (`SearchResult`, object, package, links, and publisher). The `searchScore` wire rename and every public field type are preserved. Current direct storage/search callers continue to import the legacy root model re-export.

Quản lý toàn bộ cây kết quả tìm kiếm NPM (`SearchResult`, object, package, links và publisher). Tên wire `searchScore` cùng mọi kiểu field công khai được giữ nguyên. Caller storage/search hiện tại tiếp tục import qua tái xuất model gốc cũ.
