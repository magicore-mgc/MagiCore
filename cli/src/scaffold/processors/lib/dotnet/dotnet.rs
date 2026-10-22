//! Language-specific library scaffold output.
//! Output scaffold thư viện theo ngôn ngữ.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::{slugify, write_file};

pub(in crate::scaffold::processors::lib::implementation) fn write(
    target: &Path,
    name: &str,
) -> Result<()> {
    // SDK-style csproj so NuGet audit/add lanes detect the
    // project (dotnet SDK itself stays external).
    write_file(
        &target.join(format!("{}.csproj", slugify(name))),
        concat!(
            "<Project Sdk=\"Microsoft.NET.Sdk\">\n",
            "\n",
            "  <PropertyGroup>\n",
            "    <TargetFramework>net8.0</TargetFramework>\n",
            "    <Nullable>enable</Nullable>\n",
            "  </PropertyGroup>\n",
            "\n",
            "</Project>\n"
        ),
    )?;
    write_file(
        &target.join("Class1.cs"),
        concat!(
            "namespace MagiCore.Generated;\n",
            "\n",
            "public static class Library\n",
            "{\n",
            "    public static string Greeting() => \"hello from MagiCore\";\n",
            "}\n"
        ),
    )?;
    Ok(())
}
