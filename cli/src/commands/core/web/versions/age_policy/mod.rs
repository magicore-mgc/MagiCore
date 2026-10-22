//! Web scaffold project and persisted release-age policy.
//! Security policy tuổi phát hành của project Web và store.

mod age_policy;
pub(crate) use age_policy::scaffold_age_policy;
#[cfg(all(test, feature = "web"))]
pub(in crate::commands::core::web::implementation) use age_policy::store_release_policy;
