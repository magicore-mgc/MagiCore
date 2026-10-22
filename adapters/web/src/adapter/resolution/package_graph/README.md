# Resolved package graph assembly / Dựng graph package đã resolve

Builds `ResolvedPackage` values from resolver output and registry metadata. Direct flags are derived only from indexed `RootResolution.package_id` values; a same-name transitive package cannot become a root by satisfying the manifest range. Missing root package identities return a typed error before publication.

Dựng `ResolvedPackage` từ output resolver và metadata registry. Cờ direct chỉ lấy từ `RootResolution.package_id` có index; package transitive trùng tên không thể thành root chỉ vì thỏa range manifest. Root identity bị thiếu trả lỗi typed trước khi publish. Test gần source nằm trong `test/`; mọi test vẫn `NOT_RUN`.
