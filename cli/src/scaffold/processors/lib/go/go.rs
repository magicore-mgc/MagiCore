//! Language-specific library scaffold output.
//! Output scaffold thư viện theo ngôn ngữ.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::{slugify, write_file};

pub(in crate::scaffold::processors::lib::implementation) fn write(
    target: &Path,
    name: &str,
) -> Result<()> {
    // Native lane needs a module line (detect_language +
    // go.mod writer); ship a compiling package with its own
    // test so `go test` proves the scaffold (P0 convention).
    write_file(
        &target.join("go.mod"),
        &format!("module {}\n\ngo 1.21\n", slugify(name)),
    )?;
    let package = slugify(name).replace('-', "_");
    write_file(
        &target.join("hello.go"),
        &format!(
            concat!(
                "package {package}\n",
                "\n",
                "// Hello proves the scaffold compiles.\n",
                "func Hello() string {{\n",
                "\treturn \"hello from MagiCore\"\n",
                "}}\n"
            ),
            package = package
        ),
    )?;
    write_file(
        &target.join("hello_test.go"),
        &format!(
            concat!(
                "package {package}\n",
                "\n",
                "import \"testing\"\n",
                "\n",
                "func TestHello(t *testing.T) {{\n",
                "\tif Hello() != \"hello from MagiCore\" {{\n",
                "\t\tt.Fatal(\"scaffold greeting mismatch\")\n",
                "\t}}\n",
                "}}\n"
            ),
            package = package
        ),
    )?;
    Ok(())
}
