//! Parses the existing semantic-version and supported PEP 440 forms.
//! Parse semantic-version và các dạng PEP 440 hiện được hỗ trợ.

use super::super::Version;
use super::super::is_post_or_dev_segment::is_post_or_dev_segment;
use crate::error::MgError;

impl Version {
    pub fn parse(input: &str) -> Result<Self, MgError> {
        let trimmed = input.trim().trim_start_matches('v');
        let (trimmed, build) = match trimmed.split_once('+') {
            Some((base, build))
                if !build.is_empty()
                    && build.split('.').all(|segment| {
                        !segment.is_empty()
                            && segment
                                .bytes()
                                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
                    }) =>
            {
                (base, Some(build.to_string()))
            }
            Some(_) => return Err(MgError::InvalidVersion(input.to_string())),
            None => (trimmed, None),
        };
        let (base, pre) = match trimmed.split_once('-') {
            Some((base, pre)) => (base, Some(pre.to_string())),
            None => (trimmed, None),
        };
        let mut parts = base.split('.');
        let major = parts
            .next()
            .ok_or_else(|| MgError::InvalidVersion(input.to_string()))?
            .parse()
            .map_err(|_| MgError::InvalidVersion(input.to_string()))?;
        let minor = parts
            .next()
            .unwrap_or("0")
            .parse()
            .map_err(|_| MgError::InvalidVersion(input.to_string()))?;
        // PEP 440 short form: "2.0.dev0" means "2.0.0.dev0" — a post/dev
        // segment may sit in the patch position when the patch is absent.
        // Dạng rút gọn: "2.0.dev0" tương đương "2.0.0.dev0" khi thiếu patch.
        let patch_raw = parts.next().unwrap_or("0");
        let (patch, short_extra): (u64, Option<&str>) = match patch_raw.parse() {
            Ok(patch) => (patch, None),
            Err(_) => {
                if is_post_or_dev_segment(patch_raw) {
                    (0, Some(patch_raw))
                } else {
                    return Err(MgError::InvalidVersion(input.to_string()));
                }
            }
        };
        let mut dev = None;
        let mut post = None;
        // A short-form segment from the patch position counts as the single
        // allowed extra segment — anything after it is not a version.
        // Hậu tố ở vị trí patch được tính là segment duy nhất; dữ liệu nối thêm sẽ bị từ chối.
        let short_rest: Vec<&str> = short_extra.into_iter().collect();
        let mut rest: Vec<&str> = short_rest;
        if let Some(extra) = parts.next() {
            rest.push(extra);
            rest.extend(parts);
        }
        if !rest.is_empty() {
            // This parser supports one post/dev segment (`postN`,
            // `devN`, bare `post`/`dev` = 0); more segments or any other
            // shape is not a version we understand — refuse loudly.
            // Chỉ hỗ trợ một segment post/dev; segment thừa hoặc sai dạng bị từ chối rõ ràng.
            if rest.len() != 1 {
                return Err(MgError::InvalidVersion(input.to_string()));
            }
            let segment = rest[0];
            let (is_post, digits) = if let Some(after) = segment.strip_prefix("post") {
                (true, after)
            } else if let Some(after) = segment.strip_prefix("dev") {
                (false, after)
            } else {
                return Err(MgError::InvalidVersion(input.to_string()));
            };
            let digits = if digits.is_empty() { "0" } else { digits };
            let number: u64 = digits
                .parse()
                .map_err(|_| MgError::InvalidVersion(input.to_string()))?;
            if is_post {
                post = Some(number);
            } else {
                dev = Some(number);
            }
        }
        Ok(Self {
            major,
            minor,
            patch,
            pre,
            build,
            dev,
            post,
        })
    }
}
