//! Language-specific library scaffold output.
//! Output scaffold thư viện theo ngôn ngữ.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::{slugify, write_file};

pub(in crate::scaffold::processors::lib::implementation) fn write(
    target: &Path,
    name: &str,
) -> Result<()> {
    write_file(
        &target.join("package.json"),
        &format!(
            concat!(
                "{{\n",
                "  \"name\": \"{}\",\n",
                "  \"version\": \"0.1.0\",\n",
                "  \"type\": \"module\",\n",
                "  \"scripts\": {{\n",
                "    \"test\": \"tsc --noEmit\"\n",
                "  }},\n",
                "  \"devDependencies\": {{\"typescript\": \"^5\"}}\n",
                "}}\n"
            ),
            slugify(name)
        ),
    )?;
    write_file(
        &target.join("tsconfig.json"),
        concat!(
            "{\n",
            "  \"compilerOptions\": {\n",
            "    \"target\": \"ES2022\",\n",
            "    \"module\": \"ESNext\",\n",
            "    \"declaration\": true,\n",
            "    \"outDir\": \"dist\",\n",
            "    \"rootDir\": \"src\",\n",
            "    \"strict\": true\n",
            "  },\n",
            "  \"include\": [\"src/**/*.ts\"]\n",
            "}\n"
        ),
    )?;
    write_file(
        &target.join("src").join("index.ts"),
        "export function hello(): string {\n    return 'hello from MagiCore';\n}\n",
    )?;
    Ok(())
}
