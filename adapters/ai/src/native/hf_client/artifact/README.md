# Model artifact endpoints / Endpoint tải artifact model

Owns model-file and model-card download routes plus pure endpoint assembly.
Quản lý tải file/card model cùng phần ghép endpoint thuần.

This owner preserves the public download/card methods and endpoint templates, maps local typed failures to `MgError`, and omits URLs from diagnostics.
Owner giữ method download/card public cùng endpoint template, ánh xạ lỗi cục bộ sang `MgError` và không in URL trong diagnostic.
