"""Read workflow assertions together with the scripts their steps actually invoke.
Đọc workflow và đúng script được step gọi, không thay đổi hay thực thi nội dung.
"""

from pathlib import Path
import re
import shlex


def _invoked_script_lines(root: Path, command: str, active: frozenset[Path] = frozenset()) -> list[str]:
    first = command.strip().split(maxsplit=1)
    if not first or (first[0] not in {"bash", "python", "python3", "pwsh", "powershell"}
                     and not first[0].startswith(("scripts/", "./scripts/"))):
        return []
    output = []
    for token in shlex.split(command):
        if not token.startswith(("scripts/", "./scripts/")) or not token.endswith((".sh", ".ps1", ".py")):
            continue
        script = (root / token).resolve(strict=True)
        if not script.is_relative_to(root):
            raise ValueError("workflow script escapes repository")
        if script in active:
            raise ValueError("workflow script evidence contains an invocation cycle")
        for body in script.read_text(encoding="utf-8").splitlines():
            output.append(body)
            output.extend(_invoked_script_lines(root, body, active | {script}))
    return output


def workflow_with_script_evidence(path: Path) -> str:
    root = path.parents[2].resolve()
    lines = path.read_text(encoding="utf-8").splitlines()
    output = []
    for index, line in enumerate(lines):
        output.append(line)
        match = re.match(r"^(\s*)run:\s+(.+)$", line)
        if match is None:
            continue
        indentation, command = match.groups()
        if command in {">", ">-"}:
            body = []
            for following in lines[index + 1:]:
                if following.strip() and len(following) - len(following.lstrip()) <= len(indentation):
                    break
                body.append(following.strip())
            output.append(indentation + "  " + " ".join(body))
            continue
        if command in {"|", "|-"}:
            continue
        output.extend(indentation + "  " + body for body in _invoked_script_lines(root, command))
    return "\n".join(output)
