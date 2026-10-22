//! Language-specific library scaffold output.
//! Output scaffold thư viện theo ngôn ngữ.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::{slugify, write_file};

pub(in crate::scaffold::processors::lib::implementation) fn write(
    target: &Path,
    name: &str,
) -> Result<()> {
    // Minimal pom.xml so the Maven audit/resolve lanes detect
    // the project (Maven plugins stay external).
    write_file(
        &target.join("pom.xml"),
        &format!(
            concat!(
                "<project>\n",
                "  <modelVersion>4.0.0</modelVersion>\n",
                "  <groupId>com.magicore</groupId>\n",
                "  <artifactId>{}</artifactId>\n",
                "  <version>0.1.0</version>\n",
                "  <properties>\n",
                "    <maven.compiler.source>17</maven.compiler.source>\n",
                "    <maven.compiler.target>17</maven.compiler.target>\n",
                "  </properties>\n",
                "</project>\n"
            ),
            slugify(name)
        ),
    )?;
    write_file(
        &target.join("src/main/java/dev/magicore/Library.java"),
        concat!(
            "package dev.magicore;\n",
            "\n",
            "public final class Library {\n",
            "    private Library() {}\n",
            "\n",
            "    public static String greeting() {\n",
            "        return \"hello from MagiCore\";\n",
            "    }\n",
            "}\n"
        ),
    )?;
    Ok(())
}
