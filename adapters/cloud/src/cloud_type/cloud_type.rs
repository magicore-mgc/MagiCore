//! Cloud framework identity — định danh framework Cloud.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudType {
    Cdk,
    Pulumi,
    Terraform,
    Cloudflare,
}

impl CloudType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CloudType::Cdk => "cdk",
            CloudType::Pulumi => "pulumi",
            CloudType::Terraform => "terraform",
            CloudType::Cloudflare => "cloudflare",
        }
    }
}
