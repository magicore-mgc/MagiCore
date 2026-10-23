//! Pulumi starter with a manifest contract test.
//! Starter Pulumi kèm test hợp đồng manifest.

use std::path::Path;

use anyhow::Result;

use super::super::super::{slugify, write_file};
use super::super::common::NODE_TEST_COMMAND;

pub(crate) fn write_pulumi_project(target: &Path, name: &str) -> Result<()> {
    let project_name = slugify(name);
    let package = serde_json::json!({
        "name": project_name,
        "private": true,
        "version": "0.1.0",
        "scripts": {
            "test": NODE_TEST_COMMAND
        },
        "dependencies": {
            "@pulumi/pulumi": "^3.0.0"
        }
    });
    write_file(
        &target.join("Pulumi.yaml"),
        &format!(
            "name: {project_name}\nruntime: nodejs\nmain: index.js\ndescription: MagiCore cloud project\n"
        ),
    )?;
    write_file(
        &target.join("package.json"),
        &format!("{}\n", serde_json::to_string_pretty(&package)?),
    )?;
    write_file(
        &target.join("index.js"),
        "const pulumi = require('@pulumi/pulumi');\nexports.projectName = pulumi.getProject();\n",
    )?;
    write_file(
        &target.join("test/scaffold.test.cjs"),
        "const test = require('node:test');\nconst assert = require('node:assert/strict');\nconst fs = require('node:fs');\nconst path = require('node:path');\n\ntest('Pulumi project declares a loadable program and SDK', () => {\n  const root = path.resolve(__dirname, '..');\n  const project = fs.readFileSync(path.join(root, 'Pulumi.yaml'), 'utf8');\n  const manifest = JSON.parse(fs.readFileSync(path.join(root, 'package.json'), 'utf8'));\n  assert.match(project, /^main: index\\.js$/m);\n  assert.ok(fs.statSync(path.join(root, 'index.js')).isFile());\n  assert.ok(manifest.dependencies['@pulumi/pulumi']);\n});\n",
    )
}
