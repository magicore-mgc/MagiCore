//! Cloud framework identity — định danh framework Cloud.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudType {
    Cdk,
    Pulumi,
    Terraform,
    Cloudflare,
}

impl CloudType {
    /// Parse a user-facing cloud framework name into its canonical identity.
    /// Accepts the manifest aliases the detector uses (`cdk-typescript`,
    /// `terraform-gcp`).
    /// Phân giải tên framework cloud người dùng nhập về identity chuẩn; nhận cả
    /// alias manifest mà detector dùng (`cdk-typescript`, `terraform-gcp`).
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "cdk" | "cdk-typescript" => Some(Self::Cdk),
            "pulumi" => Some(Self::Pulumi),
            "terraform" | "terraform-gcp" => Some(Self::Terraform),
            "cloudflare" => Some(Self::Cloudflare),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            CloudType::Cdk => "cdk",
            CloudType::Pulumi => "pulumi",
            CloudType::Terraform => "terraform",
            CloudType::Cloudflare => "cloudflare",
        }
    }
}
