//! Selects the capability profile from native Python lane ownership.
//! Chọn cấu hình capability theo quyền sở hữu lane Python native.

use crate::adapter::AiAdapter;
use mgc_types::capabilities::Capability;

pub(in crate::adapter::implementation) fn select_capabilities(
    adapter: &AiAdapter,
) -> &'static [Capability] {
    if adapter.python_lane.is_some() {
        AiAdapter::CAPABILITIES_PYPI
    } else {
        AiAdapter::CAPABILITIES
    }
}
