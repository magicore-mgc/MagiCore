//! Cloudflare Worker starter.
//! Starter Cloudflare Worker.

use std::path::Path;

use anyhow::Result;

use super::super::super::{slugify, write_file};

pub(crate) fn write_cloudflare(target: &Path, name: &str) -> Result<()> {
    write_file(
        &target.join("wrangler.toml"),
        &format!(
            "name = \"{}\"\nmain = \"src/index.ts\"\ncompatibility_date = \"2026-10-01\"\n",
            slugify(name)
        ),
    )?;
    write_file(
        &target.join("src/index.ts"),
        "export default {\n  async fetch(request: Request): Promise<Response> {\n    return new Response('Hello from MagiCore Worker', { status: 200 });\n  },\n};\n",
    )
}
