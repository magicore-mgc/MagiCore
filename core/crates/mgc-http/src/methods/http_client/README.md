# HttpClient owner

EN: Owns HttpClient state, construction, defaults, and Debug. Sibling owners handle request configuration, HTTP verbs, retry execution, and bounded responses. Internal state is visible only within this owner tree so those modules can share the client without changing external field visibility.

VI: Sở hữu state HttpClient, khởi tạo, giá trị mặc định và Debug. Owner cùng cấp xử lý cấu hình request, HTTP verb, retry và giới hạn response. State nội bộ chỉ mở trong cây owner để các module dùng chung client, không đổi visibility ra bên ngoài.

Known behavior: with_cache stores an HttpCache but request execution currently does not read it. See the S368 progress report; this refactor does not claim HTTP response caching works.
