mod npmrc_read;
pub(in crate::commands::config) use npmrc_read::{
    file_value, read_lines, read_optional_config_text,
};

#[cfg(test)]
mod test;
