# Cloud adapter

This owner holds the public `CloudAdapter` state, construction entry point and each capability implementation. It preserves the exact current support matrix: Cloud claims project detection, scaffold and audit; CDK/Pulumi additionally route lifecycle and content storage through the embedded Web adapter. Terraform mutation/resolution/fetch and native deployment remain fail-closed or unsupported.

Owner này chứa trạng thái `CloudAdapter` công khai, factory và từng implementation capability. Giữ nguyên ma trận hiện tại: Cloud claim nhận diện/scaffold/audit; CDK/Pulumi mới route lifecycle/content store qua Web adapter nhúng. Terraform mutation/resolution/fetch và native deploy vẫn fail-closed hoặc chưa hỗ trợ.

The public type and factory continue to be re-exported from the crate root. Adapter fields are visible only inside this private module subtree so sibling owners can implement existing traits without exposing mutable state downstream. Errors continue to use `MgError`/`MgResult`; no duplicate local error taxonomy is introduced.

Crate root tiếp tục re-export type và factory. Field adapter chỉ mở trong module subtree riêng để owner sibling implement trait hiện có, không lộ mutable state cho crate dùng bên ngoài. Lỗi tiếp tục dùng `MgError`/`MgResult`, không tạo hệ lỗi trùng lặp.

Coverage routes: `../../tests/integration.rs`, `../deploy/test/mod_test.rs`, `../scaffold/test/mod_test.rs`, plus CLI factory/dev/capability/install behavior.

Các đường test: `../../tests/integration.rs`, `../deploy/test/mod_test.rs`, `../scaffold/test/mod_test.rs` và luồng factory/dev/capability/install của CLI.
