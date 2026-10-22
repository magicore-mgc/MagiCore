//! Language-specific library scaffold output.
//! Output scaffold thư viện theo ngôn ngữ.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::{slugify, write_file};

pub(in crate::scaffold::processors::lib::implementation) fn write(
    target: &Path,
    name: &str,
) -> Result<()> {
    let package = slugify(name).replace('-', "_");
    write_file(
        &target.join("pyproject.toml"),
        &format!(
            concat!(
                "[build-system]\n",
                "requires = [\"setuptools>=80\"]\n",
                "build-backend = \"setuptools.build_meta\"\n",
                "\n",
                "[project]\n",
                "name = \"{}\"\n",
                "version = \"0.1.0\"\n",
                "requires-python = \">=3.11\"\n",
                "dependencies = []\n",
                "\n",
                "[dependency-groups]\n",
                "dev = [\"pytest==9.1.1\", \"build==1.6.1\", \"setuptools==84.0.0\"]\n"
            ),
            slugify(name)
        ),
    )?;
    write_file(
        &target.join("src").join(&package).join("__init__.py"),
        "__all__ = []\n",
    )?;
    // The scaffold ships its OWN test (P0 finding,
    // 2026-09-12): `mgc test` auto-detects pytest via
    // pyproject.toml — without a test the honest lifecycle
    // fails with pytest exit 5. The test imports the
    // package through the installed name (pip install -e
    // makes it importable); no PYTHONPATH hacks.
    // Template TỰ mang test (P0 finding, 2026-09-12):
    // `mgc test` auto-detect pytest qua pyproject.toml —
    // thiếu test thì lifecycle trung thực fail với pytest
    // exit 5. Test import package qua TÊN đã cài (pip
    // install -e cho import được); không cần PYTHONPATH.
    write_file(
        &target.join("tests").join("test_package.py"),
        &format!(
            concat!(
                "\"\"\"Scaffold smoke test: the package imports.\"\"\"\n",
                "\n",
                "import importlib\n",
                "\n",
                "\n",
                "def test_package_imports() -> None:\n",
                "    module = importlib.import_module(\"{package}\")\n",
                "    assert module.__all__ == []\n"
            ),
            package = package
        ),
    )?;
    // src-layout needs src/ on sys.path — a local conftest.py
    // does it explicitly (no `pip install -e`, no PYTHONPATH
    // hacks), so `mgc test` passes on the native lane too.
    // (conftest.py đưa src/ vào sys.path cho lane native.)
    write_file(
        &target.join("tests").join("conftest.py"),
        concat!(
            "import sys\n",
            "from pathlib import Path\n",
            "\n",
            "sys.path.insert(0, str(Path(__file__).resolve().parent.parent / \"src\"))\n"
        ),
    )?;
    Ok(())
}
