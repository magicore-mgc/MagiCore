//! Decide whether a Web project builds for the browser or for Node.
//! Quyết định project Web build cho browser hay cho Node.
//!
//! The bundler must pick an esbuild platform, and the wrong one is a hard
//! build failure: a Node backend bundled as `browser` tries to inline
//! `node:*` builtins. A project is a backend when it has no browser document
//! and at least one server runtime signal.
//! Bundler phải chọn đúng platform esbuild; chọn sai là build hỏng hẳn: backend
//! Node bundle với `browser` sẽ cố nhúng builtin `node:*`. Project là backend
//! khi không có tài liệu browser và có ít nhất một dấu hiệu runtime server.

use std::path::Path;

/// Files whose presence means the project renders a browser document.
/// File có mặt nghĩa là project render tài liệu browser.
const BROWSER_DOCUMENTS: [&str; 2] = ["index.html", "src/index.html"];

/// Server runtime signals in `package.json` dependencies.
/// Dấu hiệu runtime server trong dependency `package.json`.
const SERVER_PACKAGES: [&str; 12] = [
    "express",
    "fastify",
    "hono",
    "@hono/node-server",
    "@nestjs/core",
    "@trpc/server",
    "koa",
    "@koa/router",
    "h3",
    "connect",
    "polka",
    "restify",
];

/// Whether this project should be bundled for Node rather than the browser.
/// Project có nên bundle cho Node thay vì browser không.
pub(crate) fn is_backend_project(root: &Path) -> bool {
    if BROWSER_DOCUMENTS
        .iter()
        .any(|candidate| root.join(candidate).is_file())
    {
        return false;
    }
    let Ok(raw) = std::fs::read_to_string(root.join("package.json")) else {
        return false;
    };
    let Ok(manifest) = serde_json::from_str::<serde_json::Value>(&raw) else {
        return false;
    };
    ["dependencies", "devDependencies"]
        .iter()
        .filter_map(|key| manifest.get(*key).and_then(|value| value.as_object()))
        .any(|deps| {
            SERVER_PACKAGES
                .iter()
                .any(|package| deps.contains_key(*package))
        })
}
