//! Configure a publish client operation; this is not CLI argument storage.
//! Cấu hình thao tác của publish client; đây không phải nơi lưu CLI argument.

use serde::{Deserialize, Serialize};

/// Configure a publish client operation; the CLI builds this value from its arguments.
/// Cấu hình publish cho client; CLI tạo giá trị này từ các argument của lệnh.
#[derive(Clone, Serialize, Deserialize)]
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

// Manual Debug redacts the 2FA one-time code so a future debug/error format
// cannot leak an OTP. — Debug thủ công che mã OTP 2FA để log/panic sau này không rò.
impl std::fmt::Debug for PublishOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PublishOptions")
            .field("registry_url", &self.registry_url)
            .field("tag", &self.tag)
            .field("access", &self.access)
            .field("dry_run", &self.dry_run)
            .field(
                "otp",
                &if self.otp.is_some() {
                    "[REDACTED]"
                } else {
                    "None"
                },
            )
            .field("ignore_scripts", &self.ignore_scripts)
            .field("force", &self.force)
            .field("retries", &self.retries)
            .finish()
    }
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
