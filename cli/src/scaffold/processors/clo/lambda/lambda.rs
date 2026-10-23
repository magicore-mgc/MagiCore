//! AWS Lambda handler starter.
//! Starter handler AWS Lambda.

use std::path::Path;

use anyhow::Result;

use super::super::super::write_file;

pub(crate) fn write_lambda(target: &Path) -> Result<()> {
    write_file(
        &target.join("handler.ts"),
        "export const handler = async () => ({ statusCode: 200, body: 'ok' });\n",
    )
}
