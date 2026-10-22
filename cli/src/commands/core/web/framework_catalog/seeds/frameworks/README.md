# frameworks

Framework-specific Web package seed data — dữ liệu seed package Web theo từng framework.

Each Rust file defines exactly one `WebFrameworkSeed` constant. These leaves contain data only; they do not resolve versions, modify manifests, access registries, or choose aliases. The parent module wires the named constants, while `seeds.rs` defines their stable aggregate order.

Mỗi file Rust định nghĩa đúng một constant `WebFrameworkSeed`. Các leaf chỉ chứa dữ liệu; chúng không resolve version, sửa manifest, truy cập registry hay chọn alias. Module cha nối các constant có tên, còn `seeds.rs` quy định thứ tự aggregate ổn định.

When adding a framework, preserve the package role (`primary`, `supplemental`, or toolchain), dependency section, TypeScript-only marker, and explicit version constraint as facts from the relevant framework contract. Catalog-level contract tests live in the sibling `test/` owner.

Khi thêm framework, giữ đúng vai trò package (`primary`, `supplemental` hoặc toolchain), section dependency, cờ TypeScript-only và version constraint tường minh theo contract framework tương ứng. Test contract cấp catalog nằm trong owner `test/` cùng cấp.
