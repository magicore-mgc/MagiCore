//! Configure a publish client operation; this is not CLI argument storage.
//! Cấu hình thao tác của publish client; đây không phải nơi lưu CLI argument.

use serde::{Deserialize, Serialize};

/// Configure a publish client operation; the CLI builds this value from its arguments.
/// Cấu hình publish cho client; CLI tạo giá trị này từ các argument của lệnh.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishOptions {
    pub registry_url: String,
    #[serde(default = "default_tag")]
    pub tag: String,
    pub access: Option<String>,
    pub dry_run: bool,
    pub otp: Option<String>,
    pub ignore_scripts: bool,
    pub force: bool,
    #[serde(default = "default_retries")]
    pub retries: u32,
}

fn default_tag() -> String {
    "latest".to_string()
}

fn default_retries() -> u32 {
    3
}

impl Default for PublishOptions {
    fn default() -> Self {
        Self {
            registry_url: String::new(),
            tag: default_tag(),
            access: None,
            dry_run: false,
            otp: None,
            ignore_scripts: false,
            force: false,
            retries: default_retries(),
        }
    }
}
