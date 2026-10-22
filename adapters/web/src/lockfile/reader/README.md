# Lockfile reader owner / Owner đọc lockfile

EN: Owns the legacy reader, v4 dispatch guard, strict operation reader and owner-scoped package projection. Only publication reads the unscoped shared document.
VI: Quản lý reader legacy, cổng nhận diện v4, reader thao tác và projection package theo owner. Chỉ luồng publish lock đọc tài liệu chung chưa scope.

Preserve not-found handling, parse fallback order, diagnostics and core isolation. The local error categories map to the stable adapter `MgResult` surface.
