//! Cloudflare Worker output implementation.
//! Triển khai output Cloudflare Worker.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::cicd::implementation) fn write(target: &Path) -> Result<()> {
    write_file(
        &target.join("wrangler.toml"),
        "name = \"worker\"\nmain = \"src/index.js\"\ncompatibility_date = \"2026-01-01\"\n",
    )?;
    write_file(
        &target.join("src").join("index.js"),
        "export default {\n  async fetch(request) {\n    return new Response(\"Hello from MagiCore Worker\", { status: 200 });\n  },\n};\n",
    )?;
    Ok(())
}
