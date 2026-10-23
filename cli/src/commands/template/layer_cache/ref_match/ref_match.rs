//! Requested-ref to cached-version matching.
//! Khớp ref yêu cầu với version đã cache.

pub(crate) fn template_cache_matches_ref(
    requested: &crate::scaffold::spec::ScaffoldRef,
    cached: Option<&str>,
) -> bool {
    use crate::scaffold::spec::ScaffoldRef;
    match requested {
        ScaffoldRef::Default => false,
        ScaffoldRef::Version(version) => cached == Some(version.as_str()),
        ScaffoldRef::DistTag(_) | ScaffoldRef::Range(_) => false,
    }
}
