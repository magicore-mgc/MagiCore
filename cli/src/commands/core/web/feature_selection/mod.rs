// Web feature-selection wiring — nối logic chọn feature Web.
// Feature order remains the existing CLI order.
use super::*;
#[path = "feature_selection.rs"]
mod implementation;
pub(super) use implementation::web_features;
