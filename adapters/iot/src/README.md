# IoT adapter source root

`lib.rs` wires the public adapter, framework registry, SBOM, install, flash and scaffold modules. Domain logic belongs in those named modules; the crate root preserves the existing exports consumed by CLI and other crates.

`lib.rs` nối adapter public, framework registry, SBOM, install, flash và scaffold. Logic domain thuộc các module có tên đó; crate root giữ nguyên export đang được CLI và crate khác sử dụng.
