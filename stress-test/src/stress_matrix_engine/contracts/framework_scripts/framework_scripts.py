"""Owns the framework scripts responsibility in the stress-matrix runner.
Sở hữu trách nhiệm framework scripts trong stress-matrix runner.
"""

from __future__ import annotations

import os

import subprocess

from pathlib import Path

from ...manifest.smoke_manifest.smoke_manifest import prepare_runtime_smoke_manifest

from ...manifest.upsert_script_entries.upsert_script_entries import upsert_script_entries

from ...process.run_command.run_command import run_command

from ...result.result import Result

def run_framework_script_contracts(
    binary: Path,
    core: str,
    framework: str,
    project: Path,
    env: dict[str, str],
) -> list[Result]:
    """Exercise MGC run/test orchestration with a synthetic Python sentinel.
    Kiểm tra điều phối run/test của MGC bằng sentinel Python giả, không chạy framework.
    """
    subject = f"{core}/{framework}"
    # Use the stable allowlisted command name. Versioned absolute interpreter
    # paths (for example python3.9) are intentionally not accepted by MGC.
    # Dùng tên lệnh ổn định nằm trong allowlist; đường dẫn python3.9 tuyệt đối
    # không được MGC chấp nhận theo thiết kế.
    interpreter = "python" if os.name == "nt" else "python3"
    command = f"{interpreter} .mgc-stress-script.py"
    try:
        prepare_runtime_smoke_manifest(project, core, framework)
        script_path = project / ".mgc-stress-script.py"
        script_path.write_text(
            "import os\n"
            "print('MGC_STRESS_SCRIPT:' + os.environ.get('MGC_LIFECYCLE_EVENT', 'missing'))\n",
            encoding="utf-8",
        )
        upsert_script_entries(
            project / "mgc.toml",
            {"mgc_stress_probe": command, "test": command},
        )
    except (OSError, UnicodeError, ValueError) as error:
        return [
            Result("mgc-run-script-orchestration", subject, False, f"could not prepare script contract: {error}"),
            Result("mgc-test-script-orchestration", subject, False, f"could not prepare script contract: {error}"),
        ]

    checks = (
        (
            "mgc-run-script-orchestration",
            ["run", "--core", core, "mgc_stress_probe"],
            "MGC_STRESS_SCRIPT:mgc_stress_probe",
        ),
        (
            "mgc-test-script-orchestration",
            ["test", "--core", core],
            "MGC_STRESS_SCRIPT:test",
        ),
    )
    results: list[Result] = []
    for category, args, marker in checks:
        try:
            completed = run_command(
                [str(binary), *args], cwd=project, env=env, timeout_seconds=30
            )
            passed = completed.returncode == 0 and marker in completed.stdout
            detail = (
                "synthetic Python sentinel observed; MGC orchestration only"
                if passed
                else completed.stdout[-1400:]
            )
            results.append(Result(category, subject, passed, detail, completed.returncode))
        except (OSError, subprocess.TimeoutExpired) as error:
            results.append(Result(category, subject, False, str(error)))
    return results

__all__ = ['run_framework_script_contracts']
