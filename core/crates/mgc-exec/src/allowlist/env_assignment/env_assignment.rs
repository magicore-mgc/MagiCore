use super::env_key_validation::is_valid_env_key;
use anyhow::{Result, bail};

pub(in crate::allowlist) fn is_env_assignment(token: &str) -> bool {
    token
        .split_once('=')
        .is_some_and(|(key, _)| is_valid_env_key(key))
}

pub(in crate::allowlist) fn parse_env_assignment(token: &str) -> Result<(&str, &str)> {
    let Some((key, value)) = token.split_once('=') else {
        bail!("invalid env assignment");
    };
    if !is_valid_env_key(key) {
        bail!("invalid env assignment key '{key}'");
    }
    Ok((key, value))
}
