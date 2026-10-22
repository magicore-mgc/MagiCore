# framework_catalog

The facade preserves the stable `Scaffolder` framework methods. Web framework metadata and config inference live in [`web_framework_catalog/`](web_framework_catalog/README.md); the defaults shared across cores live in [`core_default_framework/`](core_default_framework/README.md). `mod.rs` only wires and exports the Web registry consumed by `framework_mapping`.

## Tiếng Việt

Facade giữ các method framework ổn định của `Scaffolder`. Metadata và suy config Web nằm ở [`web_framework_catalog/`](web_framework_catalog/README.md); default dùng chung nhiều core nằm ở [`core_default_framework/`](core_default_framework/README.md). `mod.rs` chỉ nối module và export registry Web cho `framework_mapping`.
