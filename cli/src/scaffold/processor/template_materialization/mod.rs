// Template materialization wiring — nối kiểm tra path và materialize theo core.
// Path verification remains explicit and shared by manifest/materializers.
use super::*;
#[path = "core_materialization/mod.rs"]
mod core_materialization;
#[path = "template_path/mod.rs"]
mod template_path;
#[path = "web_materialization/mod.rs"]
mod web_materialization;
pub(in crate::scaffold::processor) use template_path::*;
