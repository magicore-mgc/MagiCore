//! Verify scoped overrides restore both present and absent values.
//! Kiểm tra override khôi phục được cả giá trị có sẵn lẫn trạng thái vắng mặt.

use super::super::{EnvironmentOverrideGuard, lock_environment_access};
use std::ffi::OsString;

pub(crate) fn environment_override_restores_preexisting_and_absent_values() {
    const KEY: &str = "MGC_TEST_SCOPED_ENV_RESTORE";
    let _environment_lock = lock_environment_access();
    let _restore_process_value = EnvironmentOverrideGuard::remove(&_environment_lock, &[KEY]);

    {
        let _temporary =
            EnvironmentOverrideGuard::set(&_environment_lock, &[(KEY, "temporary".to_owned())]);
        assert_eq!(std::env::var_os(KEY), Some(OsString::from("temporary")));
    }
    assert_eq!(std::env::var_os(KEY), None);

    {
        let _caller_value =
            EnvironmentOverrideGuard::set(&_environment_lock, &[(KEY, "caller-value".to_owned())]);
        {
            let _temporary =
                EnvironmentOverrideGuard::set(&_environment_lock, &[(KEY, "temporary".to_owned())]);
            assert_eq!(std::env::var_os(KEY), Some(OsString::from("temporary")));
        }
        assert_eq!(std::env::var_os(KEY), Some(OsString::from("caller-value")));
    }
    assert_eq!(std::env::var_os(KEY), None);
}
