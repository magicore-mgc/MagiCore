//! Writes the CDK `package.json`, `bin` directory and `bin/app.ts` source. — Ghi `package.json` CDK, thư mục `bin` và source `bin/app.ts`.

use std::path::Path;

use mgc_types::MgResult;

pub(in crate::scaffold) async fn scaffold_cdk(name: &str, dir: &Path) -> MgResult<()> {
    let pkg = format!(
        "{{\"name\":\"{}\",\"version\":\"0.1.0\",\"dependencies\":{{\"aws-cdk-lib\":\"^2.0.0\"}}}}",
        name
    );
    std::fs::write(dir.join("package.json"), pkg)?;

    std::fs::create_dir_all(dir.join("bin"))?;
    std::fs::write(
        dir.join("bin/app.ts"),
        "import * as cdk from 'aws-cdk-lib';\n",
    )?;
    Ok(())
}
