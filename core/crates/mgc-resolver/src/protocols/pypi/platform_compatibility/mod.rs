mod platform_compatibility;
pub(in crate::protocols::pypi) use platform_compatibility::{
    host_platform_version, platform_matches_host, platform_tag_matches_target,
};
