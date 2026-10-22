//! Keep the Lib adapter's shared resolver trait implementation wired.
//! Giữ trait resolver dùng chung của LibAdapter được nối đúng.

use crate::adapter::types::LibAdapter;
use mgc_types::capabilities::DependencyResolver;

#[test]
fn lib_adapter_keeps_the_shared_dependency_resolver_contract() {
    fn assert_dependency_resolver<T: DependencyResolver>() {}

    assert_dependency_resolver::<LibAdapter>();
}
