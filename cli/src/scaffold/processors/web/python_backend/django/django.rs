//! Native Django starter.
//! Starter Django native.

use std::path::Path;

use anyhow::Result;

use super::super::super::super::write_file;
use super::super::super::common::package_name;

/// Python module name derived from the project name (Django requires an identifier).
fn module_name(name: &str) -> String {
    let mut module: String = package_name(name).replace('-', "_");
    if module.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        module.insert(0, '_');
    }
    module
}

pub(crate) fn write_django(target: &Path, name: &str) -> Result<()> {
    let package = package_name(name);
    let module = module_name(name);

    write_file(
        &target.join("pyproject.toml"),
        &format!(
            "[project]\nname = \"{package}\"\nversion = \"0.1.0\"\nrequires-python = \">=3.12\"\ndependencies = [\"django>=5.0,<6\"]\n"
        ),
    )?;

    write_file(
        &target.join("manage.py"),
        "#!/usr/bin/env python3\nimport os\nimport sys\n\n\ndef main() -> None:\n    os.environ.setdefault(\"DJANGO_SETTINGS_MODULE\", \"PROJECT.settings\")\n    from django.core.management import execute_from_command_line\n\n    execute_from_command_line(sys.argv)\n\n\nif __name__ == \"__main__\":\n    main()\n"
            .replace("PROJECT", &module)
            .as_str(),
    )?;

    write_file(&target.join(format!("{module}/__init__.py")), "")?;

    write_file(
        &target.join(format!("{module}/settings.py")),
        &format!(
            "import os\nfrom pathlib import Path\n\nBASE_DIR = Path(__file__).resolve().parent.parent\n\n# Production MUST provide DJANGO_SECRET_KEY; the fallback is development only.\n# Production BẮT BUỘC đặt DJANGO_SECRET_KEY; giá trị dự phòng chỉ dùng khi dev.\nSECRET_KEY = os.environ.get(\"DJANGO_SECRET_KEY\", \"dev-only-insecure-key\")\nDEBUG = os.environ.get(\"DJANGO_DEBUG\", \"1\") == \"1\"\nALLOWED_HOSTS = os.environ.get(\"DJANGO_ALLOWED_HOSTS\", \"*\").split(\",\")\n\nINSTALLED_APPS = [\n    \"django.contrib.contenttypes\",\n    \"django.contrib.staticfiles\",\n]\n\nMIDDLEWARE = [\n    \"django.middleware.common.CommonMiddleware\",\n]\n\nROOT_URLCONF = \"{module}.urls\"\nWSGI_APPLICATION = \"{module}.wsgi.application\"\nASGI_APPLICATION = \"{module}.asgi.application\"\n\nDATABASES = {{\n    \"default\": {{\n        \"ENGINE\": \"django.db.backends.sqlite3\",\n        \"NAME\": BASE_DIR / \"db.sqlite3\",\n    }}\n}}\n\nSTATIC_URL = \"static/\"\nDEFAULT_AUTO_FIELD = \"django.db.models.BigAutoField\"\n"
        ),
    )?;

    write_file(
        &target.join(format!("{module}/urls.py")),
        "from django.http import JsonResponse\nfrom django.urls import path\n\n\ndef health(_request):\n    return JsonResponse({\"status\": \"ok\"})\n\n\nurlpatterns = [\n    path(\"api/health\", health),\n]\n",
    )?;

    write_file(
        &target.join(format!("{module}/wsgi.py")),
        &format!(
            "import os\n\nfrom django.core.wsgi import get_wsgi_application\n\nos.environ.setdefault(\"DJANGO_SETTINGS_MODULE\", \"{module}.settings\")\n\napplication = get_wsgi_application()\n"
        ),
    )?;

    write_file(
        &target.join(format!("{module}/asgi.py")),
        &format!(
            "import os\n\nfrom django.core.asgi import get_asgi_application\n\nos.environ.setdefault(\"DJANGO_SETTINGS_MODULE\", \"{module}.settings\")\n\napplication = get_asgi_application()\n"
        ),
    )
}
