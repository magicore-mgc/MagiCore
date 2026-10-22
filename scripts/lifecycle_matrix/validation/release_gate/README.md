# Lifecycle release gate

This owner validates the generated lifecycle matrix against the source-controlled v1.2 lane, dimension, lifecycle-owner, framework-catalog and native-PM contracts. `main.py` is the CI entry point. Context/schema preparation, lane scope, dimensions, claims, global scope, failure annotations and successful summaries each have a dedicated module. The gate stays fail-closed and preserves the workflow's existing diagnostics and exit status.

## Tiếng Việt

Owner này đối chiếu matrix lifecycle được sinh ra với contract v1.2 từ source: lane, dimension, lifecycle owner, framework catalog và claim native-PM. `main.py` là entry point CI. Chuẩn bị context/schema, scope lane, dimension, claim, scope toàn cục, annotation lỗi và báo cáo thành công được tách thành module riêng. Gate vẫn fail-closed, giữ diagnostics và mã thoát hiện hành.

The local test owner belongs in `test/` during the later test phase. Until then, source-only extraction is not a test pass; workflow and cross-OS execution remain unverified.

Test cục bộ sẽ thuộc `test/` ở pha test sau. Hiện tại đây chỉ là tách source, không phải test pass; workflow và thực thi đa OS chưa được xác minh.
