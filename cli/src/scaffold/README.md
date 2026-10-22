# scaffold

Cross-core project generation, safe staging, template resolution and per-core processors.
Khởi tạo project đa core, staging an toàn, phân giải template và processor theo core.

`processor.rs` keeps the `Scaffolder` API; implementation owners are grouped under `processor/`. `container.rs` adds MGC-owned container assets after core files are written. `template_root.rs` resolves disk/registry template roots.

`spec/` owns the shared request model and parser used by scaffold callers across cores; its types, reference validation, parsing, normalization, artifact naming, and local errors are split into named owners with nearest tests.

`processor.rs` giữ API `Scaffolder`; các owner implementation nằm dưới `processor/`. `container.rs` sinh asset container do MGC quản lý sau khi ghi file core. `template_root.rs` phân giải template từ đĩa/cache registry.

`spec/` quản lý model request và parser dùng chung bởi các caller scaffold ở nhiều core; type, validation reference, parser, normalize, tên artifact và lỗi cục bộ được tách thành owner riêng với test gần implementation.
