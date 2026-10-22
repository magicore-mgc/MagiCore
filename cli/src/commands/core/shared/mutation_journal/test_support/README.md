# Mutation journal test support

Holds shared, test-only fixtures for Python project setup, adapter/lock setup, project-tree copying, and explicit journal identity construction. The module is compiled only under `cfg(test)` and is not part of product builds.

Chứa fixture chỉ dùng cho test: tạo project Python, adapter/lock, sao chép cây project và dựng identity journal. Module chỉ compile khi `cfg(test)`, không thuộc product build.
