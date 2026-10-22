"""Owns execution and result formation for one language runtime probe.
Sở hữu thực thi và tạo kết quả cho một language runtime probe.
"""

from __future__ import annotations

from pathlib import Path

from .....manifest.upsert_script_entries.upsert_script_entries import upsert_script_entries
from .....process.run_command.run_command import run_command
from .....result.result import Result

def execute_language_probe(
    binary: Path, subject: str, core: str, language: str, probe_project: Path,
    probe_env: dict[str, str], commands: tuple[str, ...], marker: str,
) -> Result:
    """Run configured steps with mgc run and preserve their result semantics.
    Chạy các bước qua mgc run và giữ nguyên ngữ nghĩa kết quả.
    """
    outputs: list[str] = []
    completed = None
    for step, command in enumerate(commands, start=1):
        script_name = f"mgc_language_runtime_probe_{step}"
        upsert_script_entries(probe_project / "mgc.toml", {script_name: command})
        completed = run_command(
            [str(binary), "run", "--core", core, script_name],
            cwd=probe_project, env=probe_env, timeout_seconds=120,
        )
        outputs.append(completed.stdout)
        if completed.returncode != 0:
            break
    output = "\n".join(outputs)
    returncode = completed.returncode if completed is not None else 1
    passed = returncode == 0 and (language == "Terraform HCL" or marker in output)
    failure_detail = (
        output if len(output) <= 3000
        else output[:1400] + "\n... output truncated ...\n" + output[-1400:]
    )
    if passed and language == "Terraform HCL":
        detail = (
            "Terraform CLI parsed and formatting-checked a dependency-free HCL file through mgc run; "
            "no provider, plan, or infrastructure operation was executed"
        )
    elif passed:
        detail = (
            f"{language} dependency-free runtime probe completed through mgc run; "
            "framework CLI and generated application source were not executed"
        )
    else:
        detail = failure_detail
    return Result(
        "framework-language-runtime-probe",
        subject,
        passed, detail, returncode, "verified" if passed else "unverified",
    )

__all__ = ["execute_language_probe"]
