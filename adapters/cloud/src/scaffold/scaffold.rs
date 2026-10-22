//! Select and run a Cloud framework generator — chọn và chạy generator framework Cloud.

use crate::scaffold::scaffold_cdk;
use crate::scaffold::scaffold_cloudflare;
use crate::scaffold::scaffold_pulumi;
use crate::scaffold::scaffold_terraform;
use std::path::Path;

use crate::cloud_type::CloudType;
use mgc_types::MgResult;

pub async fn scaffold_project(framework: CloudType, name: &str, dir: &Path) -> MgResult<()> {
    std::fs::create_dir_all(dir)?;

    match framework {
        CloudType::Cdk => scaffold_cdk(name, dir).await,
        CloudType::Pulumi => scaffold_pulumi(name, dir).await,
        CloudType::Terraform => scaffold_terraform(name, dir).await,
        CloudType::Cloudflare => scaffold_cloudflare(name, dir).await,
    }
}
