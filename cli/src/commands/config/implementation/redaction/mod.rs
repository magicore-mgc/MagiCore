mod redaction;
pub(in crate::commands::config) use redaction::{
    display_config_value, is_sensitive, redact_toml_config,
};

#[cfg(test)]
mod test;
