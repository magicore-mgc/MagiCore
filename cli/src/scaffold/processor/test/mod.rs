// Include test fragments in the parent test module scope.
// Nối các mảnh test trong cùng scope của module test cha.

include!("support.rs");
include!("generator_safety.rs");
include!("template_root.rs");
include!("delivery_descriptors.rs");
include!("web_smoke_tests.rs");
include!("core_templates.rs");
include!("web_leaf_materialization.rs");
include!("web_feature_layers.rs");
include!("multi_app.rs");
include!("scaffold_transactions.rs");
include!("iot_context.rs");
include!("manifest_path_safety.rs");
