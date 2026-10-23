//! Python Web backend starters (FastAPI, Flask).
//! Starter backend Web Python (FastAPI, Flask).

use std::path::Path;

use anyhow::Result;

use super::super::super::write_file;
use super::super::common::package_name;

pub(crate) fn write_fastapi(target: &Path, name: &str) -> Result<()> {
    let package_name = package_name(name);
    write_file(
        &target.join("pyproject.toml"),
        &format!(
            "[project]\nname = \"{package_name}\"\nversion = \"0.1.0\"\nrequires-python = \">=3.12\"\ndependencies = [\"fastapi>=0.115,<1\", \"uvicorn[standard]>=0.30,<1\"]\n"
        ),
    )?;
    write_file(
        &target.join("main.py"),
        "from fastapi import FastAPI\n\napp = FastAPI()\n\n\n@app.get(\"/api/health\")\ndef health() -> dict[str, str]:\n    return {\"status\": \"ok\"}\n",
    )
}

pub(crate) fn write_flask(target: &Path, name: &str) -> Result<()> {
    let package_name = package_name(name);
    write_file(
        &target.join("pyproject.toml"),
        &format!(
            "[project]\nname = \"{package_name}\"\nversion = \"0.1.0\"\nrequires-python = \">=3.12\"\ndependencies = [\"flask>=3.0\"]\n"
        ),
    )?;
    write_file(
        &target.join("main.py"),
        "import os\n\nfrom flask import Flask, jsonify\n\napp = Flask(__name__)\n\n\n@app.get(\"/\")\ndef index():\n    return jsonify({\"name\": \"MagiCore Flask\", \"status\": \"ok\"})\n\n\n@app.get(\"/api/health\")\ndef health():\n    return jsonify({\"status\": \"ok\"})\n\n\nif __name__ == \"__main__\":\n    app.run(host=\"0.0.0.0\", port=int(os.environ.get(\"PORT\", \"4315\")))\n",
    )
}
