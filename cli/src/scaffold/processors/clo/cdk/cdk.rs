//! AWS CDK starter with a synthesize contract test.
//! Starter AWS CDK kèm test hợp đồng synthesize.

use std::path::Path;

use anyhow::Result;

use super::super::super::{slugify, write_file};
use super::super::common::NODE_TEST_COMMAND;

pub(crate) fn write_cdk_project(target: &Path, name: &str) -> Result<()> {
    let package = serde_json::json!({
        "name": slugify(name),
        "private": true,
        "version": "0.1.0",
        "scripts": {
            "synth": "cdk synth",
            "test": NODE_TEST_COMMAND
        },
        "dependencies": {
            "aws-cdk-lib": "^2.0.0",
            "constructs": "^10.0.0"
        },
        "devDependencies": {
            "aws-cdk": "^2.0.0"
        }
    });
    write_file(
        &target.join("package.json"),
        &format!("{}\n", serde_json::to_string_pretty(&package)?),
    )?;
    write_file(
        &target.join("cdk.json"),
        "{\n  \"app\": \"node bin/app.js\"\n}\n",
    )?;
    let project_name = serde_json::to_string(&slugify(name))?;
    write_file(
        &target.join("src/app.js"),
        &format!(
            "const cdk = require('aws-cdk-lib');\n\nfunction createApp() {{\n  const app = new cdk.App();\n  const stack = new cdk.Stack(app, 'MagiCoreStack');\n  new cdk.CfnOutput(stack, 'ProjectName', {{ value: {project_name} }});\n  return {{ app, stack }};\n}}\n\nif (require.main === module) {{\n  createApp();\n}}\n\nmodule.exports = {{ createApp }};\n"
        ),
    )?;
    write_file(
        &target.join("test/scaffold.test.cjs"),
        "const test = require('node:test');\nconst { Template } = require('aws-cdk-lib/assertions');\nconst { createApp } = require('../src/app');\n\ntest('CDK app synthesizes the project output', () => {\n  const { stack } = createApp();\n  Template.fromStack(stack).hasOutput('ProjectName', {});\n});\n",
    )?;
    write_file(
        &target.join("bin/app.js"),
        "const { createApp } = require('../src/app');\ncreateApp();\n",
    )
}
