//! Reports whether the AI adapter can perform native Python updates.
//! Báo cáo adapter AI có thể update Python native hay không.

use crate::adapter::AiAdapter;

pub(in crate::adapter::implementation) fn supports_native_update(adapter: &AiAdapter) -> bool {
    adapter.python_lane.is_some()
}
