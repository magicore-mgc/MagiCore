#!/usr/bin/env python3
"""Exercise the built CLI against every framework in its capability manifest.
Chạy CLI đã build trên mọi framework được khai báo trong capability manifest.
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
import platform
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
from collections import Counter, defaultdict
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any


CORE_NAMES = ("web", "ai", "app", "lib", "game", "iot", "clo", "cicd", "hardware")
CREATE_COMMANDS = tuple(f"create-{core}" for core in CORE_NAMES)
MAX_CLI_HELP_COMMANDS = 256
DEFAULT_INVENTORY_BASELINE = (
    Path(__file__).resolve().parents[1] / "fixtures" / "framework-inventory.json"
)
# Native adapters delegate to schema-aware Web/Lib writers across these cores.
# Adapter native của các core này dùng writer Web/Lib đã nhận biết V4.
V4_INSTALL_CORES = frozenset({"web", "app", "lib", "ai", "game", "iot", "clo"})
DEPENDENCY_OPERATIONS = (
    "install",
    "add",
    "remove",
    "update",
    "list",
    "resolve",
    "lock",
    "fetch",
    "verify",
    "store",
    "materialize",
    "frozen-install",
    "offline-reinstall",
    "gc",
)
DEPENDENCY_OWNERS = frozenset({"mgc-native", "scaffold-only", "unsupported"})
FRAMEWORK_RUNTIME_CATEGORIES = (
    "framework-cli-install",
    "framework-cli-run",
    "framework-cli-test",
    "framework-cli-build",
    "framework-cli-dev",
)
WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK = {
    "nuxt": "web-fw-native-matrix",
    "vanilla": "web-fw-native-matrix",
    "ts": "web-fw-native-matrix",
    "node": "web-fw-native-matrix",
    "express": "web-fw-native-matrix",
    "fastify": "web-fw-native-matrix",
    "hono": "web-fw-native-matrix",
    "nestjs": "web-fw-native-matrix",
    "qwik": "web-fw-native-matrix",
    "solidjs": "web-fw-native-matrix",
    "trpc": "web-fw-native-matrix",
    "angular": "web-fw-frontend-template",
    "astro": "web-fw-frontend-template",
    "nextjs": "web-fw-frontend-template",
    "react-vite": "web-fw-frontend-template",
    "remix": "web-fw-frontend-template",
    "sveltekit": "web-fw-frontend-template",
    "vue-vite": "web-fw-frontend-template",
}
UNIVERSAL_FRAMEWORK_CATEGORIES = (
    "framework-scaffold",
    "framework-language-scaffold",
    "mgc-install-empty-graph",
    *FRAMEWORK_RUNTIME_CATEGORIES,
    "optimizer",
    "cache",
    "cache-runtime",
    "security-audit",
    "security-audit-nonempty",
    "shared-store-scope",
    "shared-store-runtime",
    "mgc-lock-v3-empty-graph",
    "mgc-lock-v3-nonempty",
    "mgc-lock-v4-empty-graph",
    "mgc-lock-v4-nonempty",
)
UNIVERSAL_GLOBAL_CATEGORIES = (
    "cli-command-help",
    "security-cli-command",
    "security-cli",
    "shared-store",
)
FULL_EVIDENCE_CATEGORIES = (
    "optimizer",
    "security-audit",
    "security-audit-nonempty",
)
MAX_SUPPORT_GAP_DETAILS = 40
MAX_WEB_LIFECYCLE_EVIDENCE_BYTES = 1_000_000
SOURCE_PROVENANCE_TIMEOUT_SECONDS = 5
SOURCE_SUFFIXES = {
    ".c", ".cc", ".cpp", ".cs", ".css", ".dart", ".gd", ".go", ".h",
    ".hpp", ".html", ".java", ".js", ".jsx", ".kt", ".m", ".mm", ".php",
    ".astro", ".py", ".rs", ".sh", ".swift", ".tf", ".ts", ".tsx", ".vue", ".zig",
}
SOURCE_LANGUAGE_BY_SUFFIX = {
    ".c": "C",
    ".cc": "C++",
    ".cpp": "C++",
    ".cs": "C#",
    ".css": "CSS",
    ".dart": "Dart",
    ".gd": "GDScript",
    ".go": "Go",
    ".h": "C/C++ header",
    ".hpp": "C++",
    ".html": "HTML",
    ".java": "Java",
    ".js": "JavaScript",
    ".jsx": "JavaScript",
    ".kt": "Kotlin",
    ".m": "Objective-C",
    ".mm": "Objective-C++",
    ".php": "PHP",
    ".astro": "Astro component",
    ".py": "Python",
    ".rs": "Rust",
    ".sh": "Shell",
    ".swift": "Swift",
    ".tf": "Terraform HCL",
    ".ts": "TypeScript",
    ".tsx": "TypeScript",
    ".vue": "Vue SFC",
    ".zig": "Zig",
}
AUTOMATION_PATHS = {
    ".github/workflows/ci.yml",
    "cloudbuild.yaml",
    "buildspec.yml",
    "argocd/application.yaml",
}
WEB_FRAMEWORK_ALIASES = {
    "ts": "vanilla",
    "frontend": "vanilla",
    "custom": "vanilla",
    "backend": "express",
    "node": "express",
    "fullstack": "remix",
    "go": "gin",
    "java": "spring-boot",
    "php": "laravel",
    "python": "fastapi",
    "rust": "axum",
    "dotnet": "dotnet-minimal",
}
IOT_BOARD_FRAMEWORKS = {
    "esp32": "esp32-rust",
    "esp32c3": "esp32-rust",
    "esp32s3": "esp32-rust",
    "esp32dev": "platformio",
    "nodemcu-32s": "platformio",
    "nrf52dk_nrf52832": "zephyr-arm",
    "stm32f4_disc": "zephyr-arm",
}


@dataclass
class Result:
    """One auditable command result — một kết quả lệnh có thể rà soát."""

    category: str
    subject: str
    passed: bool
    detail: str
    returncode: int | None = None
    evidence_status: str = "verified"


def run_command(
    argv: list[str],
    *,
    cwd: Path,
    env: dict[str, str],
    timeout_seconds: int,
    input_text: str = "",
) -> subprocess.CompletedProcess[str]:
    """Run one command without masking the child's exit code.
    Chạy một lệnh và giữ nguyên exit code của tiến trình con.
    """
    return subprocess.run(
        argv,
        cwd=cwd,
        env=env,
        input=input_text,
        text=True,
        encoding="utf-8",
        errors="replace",
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        timeout=timeout_seconds,
        check=False,
    )


def resolve_binary(value: str) -> Path:
    """Resolve mgc or its Windows executable name.
    Tìm binary mgc hoặc tên file thực thi trên Windows.
    """
    candidate = Path(value).expanduser().resolve()
    if candidate.is_file():
        return candidate
    if os.name == "nt" and candidate.suffix.lower() != ".exe":
        windows_candidate = candidate.with_suffix(".exe")
        if windows_candidate.is_file():
            return windows_candidate
    raise FileNotFoundError(f"MagiCore binary does not exist: {candidate}")


def isolated_environment(root: Path) -> dict[str, str]:
    """Keep all test writes inside the temporary matrix root.
    Giữ mọi dữ liệu test trong thư mục ma trận tạm.
    """
    home = root / "home"
    original_home = Path(os.environ.get("HOME") or Path.home()).expanduser()
    data = root / "system-data"
    cache = root / "cache"
    templates = root / "templates"
    temporary = root / "tmp"
    cargo_home = data / "cargo-home"
    go_cache = cache / "go-build"
    for directory in (
        home,
        data,
        cache,
        templates,
        temporary,
        cargo_home,
        go_cache,
        data / "go-mod-cache",
    ):
        directory.mkdir(parents=True, exist_ok=True)
    env = os.environ.copy()
    # The lifecycle sidecar is consumed by the parent stress runner only.
    # Sidecar chỉ dành cho runner cha, không được truyền vào script của project.
    env.pop("MGC_STRESS_LIFECYCLE_EVIDENCE", None)
    env.update(
        {
            "HOME": str(home),
            "USERPROFILE": str(home),
            # Rustup reads toolchain metadata but these probes do not mutate it.
            # (Rustup chỉ đọc metadata toolchain; các probe không sửa thư mục này.)
            "RUSTUP_HOME": os.environ.get("RUSTUP_HOME", str(original_home / ".rustup")),
            "MGC_DATA_DIR": str(data),
            "MGC_CACHE_DIR": str(cache),
            "MGC_TEMPLATES_DIR": str(templates),
            "MGC_STRESS_TEST": "1",
            "CI": "1",
            "TMPDIR": str(temporary),
            "TEMP": str(temporary),
            "TMP": str(temporary),
            "XDG_CACHE_HOME": str(cache),
            "CARGO_HOME": str(cargo_home),
            "CARGO_TARGET_DIR": str(cache / "cargo-target"),
            "GOCACHE": str(go_cache),
            "GOMODCACHE": str(data / "go-mod-cache"),
            "DOTNET_CLI_HOME": str(data / "dotnet-home"),
            "NUGET_PACKAGES": str(data / "nuget-packages"),
            "DOTNET_CLI_TELEMETRY_OPTOUT": "1",
            "DOTNET_SKIP_FIRST_TIME_EXPERIENCE": "1",
        }
    )
    return env


def capability_rows(binary: Path, root: Path, env: dict[str, str]) -> tuple[str, list[dict[str, Any]]]:
    """Read and validate the CLI's own framework inventory.
    Đọc và kiểm tra danh mục framework do chính CLI công bố.
    """
    completed = run_command(
        [str(binary), "capabilities", "--json"],
        cwd=root,
        env=env,
        timeout_seconds=30,
    )
    if completed.returncode != 0:
        raise RuntimeError(f"mgc capabilities --json failed ({completed.returncode}): {completed.stdout[-1200:]}")
    try:
        payload = json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise RuntimeError(f"mgc capabilities emitted invalid JSON: {error}") from error
    if not isinstance(payload, dict):
        raise RuntimeError("mgc capabilities JSON root must be an object")
    version = payload.get("version")
    core_records = payload.get("cores")
    if not isinstance(version, str) or not isinstance(core_records, list):
        raise RuntimeError("capability manifest must contain version and cores")

    actual_cores: set[str] = set()
    rows: list[dict[str, Any]] = []
    for core_record in core_records:
        if not isinstance(core_record, dict):
            raise RuntimeError("capability manifest contains a malformed core row")
        core = core_record.get("core")
        frameworks = core_record.get("framework_qualification")
        if not isinstance(core, str) or not isinstance(frameworks, list):
            raise RuntimeError("capability core row is missing framework_qualification")
        if core in actual_cores:
            raise RuntimeError(f"duplicate core in capability manifest: {core}")
        actual_cores.add(core)
        for framework_row in frameworks:
            if not isinstance(framework_row, dict):
                raise RuntimeError(f"core {core} contains a malformed framework row")
            framework = framework_row.get("framework")
            status = framework_row.get("status")
            if not isinstance(framework, str) or not framework.strip():
                raise RuntimeError(f"invalid framework row for core {core}")
            if not isinstance(status, str) or not status:
                raise RuntimeError(f"framework {core}/{framework} has no qualification status")
            ownership = framework_row.get("dependency_ownership")
            if not isinstance(ownership, dict):
                raise RuntimeError(f"framework {core}/{framework} has no dependency ownership table")
            missing_operations = sorted(set(DEPENDENCY_OPERATIONS) - set(ownership))
            unknown_operations = sorted(set(ownership) - set(DEPENDENCY_OPERATIONS))
            if missing_operations or unknown_operations:
                details = []
                if missing_operations:
                    details.append("missing " + ", ".join(missing_operations))
                if unknown_operations:
                    details.append("unknown " + ", ".join(unknown_operations))
                raise RuntimeError(
                    f"framework {core}/{framework} dependency ownership operations are incomplete: "
                    + "; ".join(details)
                )
            normalized_ownership: dict[str, dict[str, Any]] = {}
            for operation in DEPENDENCY_OPERATIONS:
                cell = ownership[operation]
                owner = cell.get("owner") if isinstance(cell, dict) else None
                if not isinstance(owner, str) or owner not in DEPENDENCY_OWNERS:
                    raise RuntimeError(
                        f"framework {core}/{framework} operation {operation} has invalid owner {owner!r}"
                    )
                normalized_ownership[operation] = dict(cell)
            rows.append(
                {
                    "core": core,
                    "framework": framework,
                    "status": status,
                    "dependency_ownership": normalized_ownership,
                }
            )

    missing = sorted(set(CORE_NAMES) - actual_cores)
    if missing:
        raise RuntimeError(f"capability manifest omitted core(s): {', '.join(missing)}")
    unexpected = sorted(actual_cores - set(CORE_NAMES))
    if unexpected:
        raise RuntimeError(
            f"capability manifest introduced unsupported core(s): {', '.join(unexpected)}"
        )
    if not rows:
        raise RuntimeError("capability manifest contains no framework rows")
    unique = {(row["core"], row["framework"]) for row in rows}
    if len(unique) != len(rows):
        raise RuntimeError("capability manifest contains duplicate core/framework rows")
    return version, rows


def check_framework_inventory(rows: list[dict[str, Any]], baseline_path: Path) -> int:
    """Reject silent framework, qualification, or operation-owner drift.
    Từ chối thay đổi âm thầm danh sách framework, qualification hay owner.
    """
    try:
        baseline = json.loads(baseline_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise RuntimeError(f"cannot read framework inventory baseline: {error}") from error
    if not isinstance(baseline, dict):
        raise RuntimeError("framework inventory baseline has an unsupported schema")
    schema_version = baseline.get("schema_version")
    if (
        not isinstance(schema_version, int)
        or isinstance(schema_version, bool)
        or schema_version != 1
    ):
        raise RuntimeError("framework inventory baseline has an unsupported schema")
    expected_rows = baseline.get("frameworks")
    if not isinstance(expected_rows, list) or not expected_rows:
        raise RuntimeError("framework inventory baseline must contain framework rows")
    captured_from = baseline.get("captured_from")
    if not isinstance(captured_from, dict):
        raise RuntimeError("framework inventory baseline has no capture metadata")
    if (
        not isinstance(captured_from.get("mgc_version"), str)
        or not captured_from["mgc_version"].strip()
        or not isinstance(captured_from.get("git_head"), str)
        or not captured_from["git_head"].strip()
        or not isinstance(captured_from.get("captured_at"), str)
        or not captured_from["captured_at"].strip()
        or not isinstance(captured_from.get("framework_count"), int)
        or isinstance(captured_from.get("framework_count"), bool)
        or captured_from.get("framework_count") != len(expected_rows)
    ):
        raise RuntimeError("framework inventory baseline capture metadata is incomplete or inconsistent")

    def normalize(items: list[dict[str, Any]], source: str) -> dict[tuple[str, str], dict[str, Any]]:
        if not isinstance(items, list):
            raise RuntimeError(f"{source} rows must be a list")
        normalized: dict[tuple[str, str], dict[str, Any]] = {}
        for item in items:
            if not isinstance(item, dict):
                raise RuntimeError(f"{source} contains a malformed row")
            core = item.get("core")
            framework = item.get("framework")
            status = item.get("status")
            owners = item.get("dependency_ownership")
            if (
                not isinstance(core, str)
                or not core.strip()
                or not isinstance(framework, str)
                or not framework.strip()
            ):
                raise RuntimeError(f"{source} has an invalid core/framework identity")
            if not isinstance(status, str) or not status:
                raise RuntimeError(f"{source} row {core}/{framework} has no status")
            if not isinstance(owners, dict) or set(owners) != set(DEPENDENCY_OPERATIONS):
                raise RuntimeError(f"{source} row {core}/{framework} has an incomplete owner table")
            if any(
                not isinstance(owner, str) or owner not in DEPENDENCY_OWNERS
                for owner in owners.values()
            ):
                raise RuntimeError(f"{source} row {core}/{framework} has an unknown owner")
            identity = (core, framework)
            if identity in normalized:
                raise RuntimeError(f"{source} contains duplicate row {core}/{framework}")
            normalized[identity] = {"status": status, "owners": owners}
        return normalized

    expected = normalize(expected_rows, "framework inventory baseline")
    actual_rows = []
    for row in rows:
        if not isinstance(row, dict):
            raise RuntimeError("capability manifest contains a malformed framework row")
        ownership = row.get("dependency_ownership")
        if not isinstance(ownership, dict):
            raise RuntimeError("capability manifest contains a malformed owner table")
        actual_owners = {}
        for operation, cell in ownership.items():
            if not isinstance(cell, dict):
                raise RuntimeError("capability manifest contains a malformed owner cell")
            actual_owners[operation] = cell.get("owner")
        actual_rows.append(
            {
                "core": row.get("core"),
                "framework": row.get("framework"),
                "status": row.get("status"),
                "dependency_ownership": actual_owners,
            }
        )
    actual = normalize(
        actual_rows,
        "capability manifest",
    )
    missing = sorted(expected.keys() - actual.keys())
    unexpected = sorted(actual.keys() - expected.keys())
    if missing or unexpected:
        details = []
        if missing:
            details.append("missing " + ", ".join(f"{core}/{framework}" for core, framework in missing[:20]))
        if unexpected:
            details.append("unexpected " + ", ".join(f"{core}/{framework}" for core, framework in unexpected[:20]))
        raise RuntimeError("framework inventory drift: " + "; ".join(details))

    changes = []
    for identity in sorted(expected):
        before = expected[identity]
        after = actual[identity]
        label = f"{identity[0]}/{identity[1]}"
        if before["status"] != after["status"]:
            changes.append(f"{label} qualification status {before['status']} -> {after['status']}")
        for operation in DEPENDENCY_OPERATIONS:
            previous_owner = before["owners"][operation]
            current_owner = after["owners"][operation]
            if previous_owner != current_owner:
                changes.append(
                    f"{label} {operation} owner {previous_owner} -> {current_owner}"
                )
    if changes:
        raise RuntimeError("framework inventory contract changed: " + "; ".join(changes[:20]))
    return len(expected)


def project_slug(core: str, framework: str) -> str:
    """Make a portable, single-segment project name.
    Tạo tên project một segment, dùng được trên các hệ điều hành.
    """
    safe = re.sub(r"[^a-zA-Z0-9]+", "-", framework).strip("-").lower()
    return f"mgc-{core}-{safe or 'framework'}"


def has_payload(project: Path, core: str) -> bool:
    """Require source code, or an actual CI/CD definition, beyond MGC metadata.
    Bắt buộc có source code hoặc pipeline CI/CD thực ngoài metadata MGC.
    """
    for path in project.rglob("*"):
        relative_parts = path.relative_to(project).parts
        if not path.is_file() or any(
            part in {".git", ".magicore"} or part.startswith(".mgc")
            for part in relative_parts
        ):
            continue
        if path.name in {"mgc.toml", ".mgc.core", ".mgc-provenance.json"}:
            continue
        relative = path.relative_to(project).as_posix().lower()
        is_cicd_definition = core == "cicd" and (
            relative in AUTOMATION_PATHS
            or (relative.startswith(".github/workflows/") and path.suffix.lower() in {".yaml", ".yml"})
        )
        if core == "hardware" and path.name in {"bench.json", "optimizer.json"}:
            try:
                descriptor = json.loads(path.read_text(encoding="utf-8"))
            except (OSError, UnicodeError, json.JSONDecodeError):
                continue
            expected_kind = path.stem
            if (
                isinstance(descriptor, dict)
                and descriptor.get("kind") == expected_kind
                and isinstance(descriptor.get("name"), str)
                and descriptor["name"].strip()
            ):
                return True
        if path.suffix.lower() in SOURCE_SUFFIXES or is_cicd_definition:
            try:
                if path.stat().st_size > 0:
                    return True
            except OSError:
                continue
    return False


def validate_scaffold(project: Path, core: str, framework: str) -> str | None:
    """Check identity and generated project payload, not just command success.
    Kiểm tra identity và file project sinh ra, không chỉ exit code.
    """
    if not project.is_dir():
        return "project directory was not created"
    marker = project / ".mgc.core"
    if not marker.is_file() or marker.read_text(encoding="utf-8").strip() != core:
        return "missing or incorrect .mgc.core marker"
    config_path = project / "mgc.toml"
    if not config_path.is_file():
        return "mgc.toml was not created"
    try:
        config = read_project_identity(config_path)
    except (OSError, ValueError) as error:
        return f"mgc.toml identity is invalid: {error}"
    if config.get("ecosystem") != core and not (core == "clo" and config.get("ecosystem") == "cloud"):
        return f"mgc.toml ecosystem is {config.get('ecosystem')!r}, expected {core!r}"
    frameworks = config.get("frameworks")
    identity_matches = isinstance(frameworks, list) and framework in frameworks
    if core == "web" and isinstance(frameworks, list) and not identity_matches:
        canonical = WEB_FRAMEWORK_ALIASES.get(framework)
        if canonical is not None:
            identity_matches = canonical in frameworks
        elif framework == "monorepo":
            identity_matches = (
                config.get("mode") == "monorepo"
                and len(frameworks) >= 2
                and "react-vite" in frameworks
            )
    if core == "iot" and framework in IOT_BOARD_FRAMEWORKS:
        identity_matches = (
            config.get("iot_framework") == IOT_BOARD_FRAMEWORKS[framework]
            and config.get("iot_board") == framework
        )
    if not identity_matches:
        return f"mgc.toml framework identity does not include {framework!r}: {frameworks!r}"
    if not has_payload(project, core):
        return "scaffold contains no non-empty source file or CI/CD definition"
    return None


def read_project_identity(config_path: Path) -> dict[str, Any]:
    """Read the generated top-level identity fields without third-party TOML packages.
    Đọc các trường identity đầu file mà không cần package TOML bên ngoài.
    """
    lines = config_path.read_text(encoding="utf-8").splitlines()
    top_level: list[str] = []
    for line in lines:
        if line.lstrip().startswith("["):
            break
        top_level.append(line)

    values: dict[str, Any] = {}
    for index, line in enumerate(top_level):
        match = re.match(r"^\s*(ecosystem|mode|frameworks)\s*=\s*(.*?)\s*$", line)
        if not match:
            continue
        key, raw = match.groups()
        if key in values:
            raise ValueError(f"duplicate {key} field")
        if key in {"ecosystem", "mode"}:
            values[key] = parse_toml_string(raw)
            continue
        while raw.count("[") > raw.count("]"):
            index += 1
            if index >= len(top_level):
                raise ValueError("unterminated frameworks array")
            raw += top_level[index].strip()
        values[key] = parse_toml_string_array(raw)
    section: str | None = None
    for line in lines:
        header = re.match(r"^\s*\[([^]]+)\]\s*$", line)
        if header:
            section = header.group(1)
            continue
        if section != "iot":
            continue
        field = re.match(r"^\s*(framework|board)\s*=\s*(.*?)\s*$", line)
        if field:
            key, raw = field.groups()
            target = "iot_framework" if key == "framework" else "iot_board"
            if target in values:
                raise ValueError(f"duplicate {target} field")
            values[target] = parse_toml_string(raw)
    if not values:
        raise ValueError("top-level ecosystem/frameworks fields are missing")
    return values


def parse_toml_string(raw: str) -> str:
    """Decode a basic TOML single- or double-quoted string.
    Giải mã chuỗi TOML cơ bản dùng nháy đơn hoặc nháy kép.
    """
    value = raw.strip()
    if len(value) < 2 or value[0] not in {"'", '"'} or value[-1] != value[0]:
        raise ValueError("expected a quoted TOML string")
    if value[0] == "'":
        return value[1:-1]
    decoded = json.loads(value)
    if not isinstance(decoded, str):
        raise ValueError("expected a TOML string")
    return decoded


def parse_toml_string_array(raw: str) -> list[str]:
    """Decode a generated TOML array containing only quoted strings.
    Giải mã mảng TOML do MGC sinh, chỉ gồm các chuỗi được trích dẫn.
    """
    value = raw.strip()
    if not value.startswith("[") or not value.endswith("]"):
        raise ValueError("expected a TOML array")
    items: list[str] = []
    index = 1
    decoder = json.JSONDecoder()
    while index < len(value) - 1:
        while index < len(value) - 1 and (value[index].isspace() or value[index] == ","):
            index += 1
        if index >= len(value) - 1:
            break
        if value[index] == "'":
            end = value.find("'", index + 1)
            if end < 0:
                raise ValueError("unterminated literal string in frameworks array")
            items.append(value[index + 1:end])
            index = end + 1
        elif value[index] == '"':
            item, consumed = decoder.raw_decode(value[index:])
            if not isinstance(item, str):
                raise ValueError("frameworks array must contain strings")
            items.append(item)
            index += consumed
        else:
            raise ValueError("frameworks array contains a non-string value")
        while index < len(value) - 1 and value[index].isspace():
            index += 1
        if index < len(value) - 1 and value[index] != ",":
            raise ValueError("expected a comma between framework strings")
    return items


def upsert_script_entries(config_path: Path, entries: dict[str, str]) -> None:
    """Add deterministic CLI smoke scripts without duplicating TOML keys.
    Thêm script smoke CLI xác định, không tạo key TOML trùng.
    """
    lines = config_path.read_text(encoding="utf-8").splitlines()
    section_starts = [
        index
        for index, line in enumerate(lines)
        if re.fullmatch(r"\s*\[scripts\]\s*(?:#.*)?", line)
    ]
    if len(section_starts) > 1:
        raise ValueError("mgc.toml contains more than one [scripts] table")

    if not section_starts:
        if re.search(r"^\s*scripts\s*=", "\n".join(lines), re.MULTILINE):
            raise ValueError("mgc.toml uses an inline scripts value that the stress runner cannot extend")
        if lines and lines[-1].strip():
            lines.append("")
        lines.append("[scripts]")
        lines.extend(f"{key} = {json.dumps(command)}" for key, command in entries.items())
        config_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
        return

    section_start = section_starts[0]
    section_end = next(
        (
            index
            for index in range(section_start + 1, len(lines))
            if re.match(r"\s*\[[^]]+\]", lines[index])
        ),
        len(lines),
    )
    for key, command in entries.items():
        key_pattern = re.compile(rf"^\s*{re.escape(key)}\s*=")
        matches = [
            index
            for index in range(section_start + 1, section_end)
            if key_pattern.match(lines[index])
        ]
        replacement = f"{key} = {json.dumps(command)}"
        if len(matches) > 1:
            raise ValueError(f"mgc.toml contains duplicate [scripts].{key} entries")
        if matches:
            lines[matches[0]] = replacement
        else:
            lines.insert(section_end, replacement)
            section_end += 1
    config_path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def set_lock_schema(config_path: Path, schema: str) -> None:
    """Pin a generated project to one lock writer for deterministic evidence.
    Ghim project sinh ra vào một lock writer để bằng chứng xác định.
    """
    if schema not in {"3", "v4"}:
        raise ValueError(f"unsupported stress lock schema: {schema}")

    lines = config_path.read_text(encoding="utf-8").splitlines()
    lock_headers = [
        index
        for index, line in enumerate(lines)
        if re.fullmatch(r"\s*\[lock\]\s*(?:#.*)?", line)
    ]
    if len(lock_headers) > 1:
        raise ValueError("mgc.toml contains more than one [lock] table")
    if not lock_headers:
        if re.search(r"^\s*lock\s*=", "\n".join(lines), re.MULTILINE):
            raise ValueError("mgc.toml uses an inline lock value that the stress runner cannot extend")
        if lines and lines[-1].strip():
            lines.append("")
        lines.extend(("[lock]", f"schema = {json.dumps(schema)}"))
        config_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
        return

    table_start = lock_headers[0]
    table_end = next(
        (
            index
            for index in range(table_start + 1, len(lines))
            if re.match(r"\s*\[[^]]+\]", lines[index])
        ),
        len(lines),
    )
    schema_keys = [
        index
        for index in range(table_start + 1, table_end)
        if re.match(r"\s*schema\s*=", lines[index])
    ]
    if len(schema_keys) > 1:
        raise ValueError("mgc.toml contains duplicate [lock].schema entries")
    replacement = f"schema = {json.dumps(schema)}"
    if schema_keys:
        lines[schema_keys[0]] = replacement
    else:
        lines.insert(table_end, replacement)
    config_path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def lockfile_version(lock_path: Path) -> int | None:
    """Read the top-level lock schema major without accepting partial text.
    Đọc major schema đầu file, gồm cả chuỗi revision V4 như ``4.1``.
    """
    try:
        text = lock_path.read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return None
    match = re.search(
        r"""(?m)^version\s*=\s*(?:"(\d+(?:\.\d+)*)"|'(\d+(?:\.\d+)*)'|(\d+))\s*(?:#.*)?$""",
        text,
    )
    if match is None:
        return None
    revision = next(value for value in match.groups() if value is not None)
    return int(revision.split(".", maxsplit=1)[0])


def run_lockfile_contracts(
    binary: Path,
    row: dict[str, Any],
    project: Path,
    env: dict[str, str],
    install_succeeded: bool,
) -> list[Result]:
    """Verify native lock replay per framework and V4 where its writer is wired.
    Kiểm tra replay lock native theo từng framework và V4 tại core đã nối writer.
    """
    core = row["core"]
    framework = row["framework"]
    subject = f"{core}/{framework}"
    owner = row["dependency_ownership"]["install"]["owner"]
    if owner != "mgc-native":
        return [
            Result(
                "mgc-lock-v3-empty-graph",
                subject,
                True,
                f"not applicable: install owner is {owner}",
                evidence_status="unsupported",
            ),
            Result(
                "mgc-lock-v4-empty-graph",
                subject,
                True,
                f"not applicable: install owner is {owner}",
                evidence_status="unsupported",
            ),
        ]

    lock_path = project / "mgc.lock"
    config_path = project / "mgc.toml"
    if not install_succeeded:
        v3 = Result(
            "mgc-lock-v3-empty-graph",
            subject,
            True,
            "not verified because the declared native install did not complete",
            evidence_status="unverified",
        )
    elif lockfile_version(lock_path) != 3:
        v3 = Result(
            "mgc-lock-v3-empty-graph",
            subject,
            False,
            f"native V3 install did not produce a version 3 mgc.lock (found {lockfile_version(lock_path)!r})",
        )
    else:
        original_v3 = lock_path.read_bytes()
        try:
            replay = run_command(
                [str(binary), f"install-{core}", "--frozen"],
                cwd=project,
                env=env,
                timeout_seconds=120,
            )
            replay_stable = lock_path.read_bytes() == original_v3
            passed = replay.returncode == 0 and replay_stable
            detail = (
                "empty-graph V3 frozen install replayed and preserved lock bytes"
                if passed
                else "empty-graph V3 frozen replay failed or changed lock bytes: " + replay.stdout[-1200:]
            )
            v3 = Result("mgc-lock-v3-empty-graph", subject, passed, detail, replay.returncode)
        except (OSError, subprocess.TimeoutExpired) as error:
            v3 = Result("mgc-lock-v3-empty-graph", subject, False, str(error))

    if core not in V4_INSTALL_CORES:
        v4 = Result(
            "mgc-lock-v4-empty-graph",
            subject,
            True,
            f"unsupported: no native V4 install writer is wired for the {core} core",
            evidence_status="unsupported",
        )
        return [v3, v4]
    if not install_succeeded:
        return [
            v3,
            Result(
                "mgc-lock-v4-empty-graph",
                subject,
                True,
                "not verified because the declared native install did not complete",
                evidence_status="unverified",
            ),
        ]

    try:
        set_lock_schema(config_path, "v4")
        write = run_command(
            [str(binary), f"install-{core}"],
            cwd=project,
            env=env,
            timeout_seconds=120,
        )
        version = lockfile_version(lock_path)
        if write.returncode != 0 or version != 4:
            v4 = Result(
                "mgc-lock-v4-empty-graph",
                subject,
                False,
                f"V4 install exited {write.returncode}, lock version is {version!r}: {write.stdout[-1200:]}",
                write.returncode,
            )
        else:
            original_v4 = lock_path.read_bytes()
            replay = run_command(
                [str(binary), f"install-{core}", "--frozen"],
                cwd=project,
                env=env,
                timeout_seconds=120,
            )
            replay_stable = lock_path.read_bytes() == original_v4
            passed = replay.returncode == 0 and replay_stable and lockfile_version(lock_path) == 4
            detail = (
                "empty-graph V4 frozen install replayed and preserved lock bytes"
                if passed
                else "empty-graph V4 frozen replay failed or changed lock bytes: " + replay.stdout[-1200:]
            )
            v4 = Result("mgc-lock-v4-empty-graph", subject, passed, detail, replay.returncode)
    except (OSError, UnicodeError, ValueError, subprocess.TimeoutExpired) as error:
        v4 = Result("mgc-lock-v4-empty-graph", subject, False, str(error))
    return [v3, v4]


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


RUNTIME_PROBE_LANGUAGE_ORDER = (
    "TypeScript",
    "JavaScript",
    "Python",
    "Go",
    "Dart",
    "Rust",
    "Swift",
    "C",
    "C++",
    "C#",
    "Java",
    "Kotlin",
    "PHP",
    "Shell",
    "GDScript",
    "Objective-C",
    "Objective-C++",
    "Zig",
    "Terraform HCL",
)
NON_STANDALONE_RUNTIME_FORMATS = frozenset(
    {
        "Astro component",
        "C/C++ header",
        "CI workflow YAML",
        "CSS",
        "HTML",
        "JSON hardware descriptor",
        "Vue SFC",
    }
)


def language_runtime_profile(
    language: str,
) -> tuple[str, str, str, tuple[str, ...]] | None:
    """Return a dependency-free MagiCore-run probe for qualified languages.
    Trả probe độc lập dependency để chạy runtime ngôn ngữ qua MagiCore.
    """
    marker = f"MGC_STRESS_LANGUAGE_RUNTIME:{language}"
    if language == "TypeScript":
        filename = ".mgc-stress-language.ts"
        source = f'const evidence: string = "{marker}";\nconsole.log(evidence);\n'
        return "node", filename, source, (f"node --experimental-strip-types {filename}",)
    if language == "JavaScript":
        filename = ".mgc-stress-language.mjs"
        source = f'console.log("{marker}");\n'
        return "node", filename, source, (f"node {filename}",)
    if language == "Python":
        filename = ".mgc-stress-language.py"
        source = f'print("{marker}")\n'
        interpreter = "python" if os.name == "nt" else "python3"
        return interpreter, filename, source, (f"{interpreter} {filename}",)
    if language == "Go":
        filename = "mgc_stress_language.go"
        source = (
            "package main\n\nimport \"fmt\"\n\n"
            f"func main() {{ fmt.Println(\"{marker}\") }}\n"
        )
        return "go", filename, source, (f"GOPROXY=off GOSUMDB=off GOTOOLCHAIN=local go run -mod=readonly {filename}",)
    if language == "Dart":
        filename = ".mgc-stress-language.dart"
        source = f'void main() {{ print("{marker}"); }}\n'
        return "dart", filename, source, (f"dart {filename}",)
    if language == "Rust":
        filename = ".mgc-stress-runtime-rust/Cargo.toml"
        source = (
            "[package]\nname = \"mgc-stress-runtime-probe\"\n"
            "version = \"0.0.0\"\nedition = \"2021\"\n"
        )
        command = f"cargo run --quiet --locked --offline --manifest-path {filename}"
        return "cargo", filename, source, (command,)
    if language == "Swift":
        filename = ".mgc-stress-language.swift"
        source = f'print("{marker}")\n'
        command = (
            "swift -module-cache-path .mgc-swift-module-cache "
            "-sdk-module-cache-path .mgc-swift-sdk-module-cache "
            f"{filename}"
        )
        return "swift", filename, source, (command,)
    if language == "Zig":
        filename = ".mgc-stress-language.zig"
        source = (
            'const std = @import("std");\n'
            "pub fn main() void {\n"
            f'    std.debug.print("{marker}\\n", .{{}});\n'
            "}\n"
        )
        return "zig", filename, source, (f"zig run {filename}",)
    if language == "C":
        filename = ".mgc-stress-language.c"
        source = (
            "#include <stdio.h>\n"
            f'int main(void) {{ puts("{marker}"); return 0; }}\n'
        )
        output = ".mgc-stress-language-c.exe" if os.name == "nt" else ".mgc-stress-language-c"
        executable_path = f"./{output}"
        compiler = "clang" if sys.platform == "darwin" else "gcc"
        return (
            compiler,
            filename,
            source,
            (
                f"{compiler} {filename} -o {output}",
                executable_path,
            ),
        )
    if language == "C++":
        filename = ".mgc-stress-language.cpp"
        source = (
            "#include <iostream>\n"
            f'int main() {{ std::cout << "{marker}" << std::endl; return 0; }}\n'
        )
        output = ".mgc-stress-language-cpp.exe" if os.name == "nt" else ".mgc-stress-language-cpp"
        executable_path = f"./{output}"
        compiler = "clang++" if sys.platform == "darwin" else "g++"
        return (
            compiler,
            filename,
            source,
            (
                f"{compiler} {filename} -o {output}",
                executable_path,
            ),
        )
    if language == "C#":
        filename = ".mgc-stress-dotnet/Program.cs"
        source = f'Console.WriteLine("{marker}");\n'
        command = "dotnet exec .mgc-stress-dotnet/bin/Debug/net10.0/MgcStressProbe.dll"
        return "dotnet", filename, source, (command,)
    if language == "Java":
        filename = "MgcStressLanguageProbe.java"
        source = (
            "class MgcStressLanguageProbe {\n"
            "  public static void main(String[] args) {\n"
            f'    System.out.println("{marker}");\n'
            "  }\n"
            "}\n"
        )
        command = (
            "java -XX:-UsePerfData -Djava.io.tmpdir=.mgc-java-tmp "
            f"{filename}"
        )
        return "java", filename, source, (command,)
    if language == "Kotlin":
        filename = ".mgc-stress-language.kt"
        source = f'fun main() {{ println("{marker}") }}\n'
        jar = ".mgc-stress-language-kotlin.jar"
        compile_command = f"kotlinc {filename} -include-runtime -d {jar}"
        execute_command = (
            f"java -XX:-UsePerfData -Djava.io.tmpdir=.mgc-java-tmp -jar {jar}"
        )
        return "kotlinc", filename, source, (compile_command, execute_command)
    if language == "PHP":
        filename = ".mgc-stress-language.php"
        source = f'<?php echo "{marker}\\n";\n'
        return "php", filename, source, (f"php -n {filename}",)
    if language == "Shell":
        filename = ".mgc-stress-language.sh"
        source = f'#!/usr/bin/env bash\nprintf \'%s\\n\' "{marker}"\n'
        return "bash", filename, source, (f"bash {filename}",)
    if language == "GDScript":
        filename = ".mgc-stress-godot/probe.gd"
        source = (
            "extends SceneTree\n"
            "func _initialize():\n"
            f'    print("{marker}")\n'
            "    quit()\n"
        )
        command = (
            "godot --headless --path .mgc-stress-godot "
            "--script probe.gd --quit"
        )
        return "godot", filename, source, (command,)
    if language == "Terraform HCL":
        filename = ".mgc-stress-language.tf"
        source = (
            'output "mgc_stress_language_runtime" {\n'
            f'  value = "{marker}"\n'
            '}\n'
        )
        return "terraform", filename, source, (f"terraform fmt -check {filename}",)
    if language == "Objective-C" and sys.platform == "darwin":
        filename = ".mgc-stress-language.m"
        source = (
            "#import <Foundation/Foundation.h>\n"
            "#include <stdio.h>\n"
            "int main(void) { @autoreleasepool { "
            f'printf("%s\\n", "{marker}"); '
            "} return 0; }\n"
        )
        output = ".mgc-stress-language-objc"
        return (
            "clang",
            filename,
            source,
            (
                f"clang -x objective-c {filename} -framework Foundation -o {output}",
                f"./{output}",
            ),
        )
    if language == "Objective-C++" and sys.platform == "darwin":
        filename = ".mgc-stress-language.mm"
        source = (
            "#import <Foundation/Foundation.h>\n"
            "#include <iostream>\n"
            "int main() { @autoreleasepool { "
            f'std::cout << "{marker}" << std::endl; '
            "} return 0; }\n"
        )
        output = ".mgc-stress-language-objcxx"
        return (
            "clang++",
            filename,
            source,
            (
                f"clang++ -x objective-c++ {filename} -framework Foundation -o {output}",
                f"./{output}",
            ),
        )
    return None


def run_framework_language_runtime_probe(
    binary: Path,
    core: str,
    framework: str,
    project: Path,
    env: dict[str, str],
) -> list[Result]:
    """Run each standalone source-language probe through mgc run.
    """
    subject = f"{core}/{framework}"
    language_files, _ = scaffold_language_files(project, core)
    languages = [
        candidate
        for candidate in RUNTIME_PROBE_LANGUAGE_ORDER
        if language_files.get(candidate, 0) > 0
    ]
    if not languages:
        return [
            Result(
                "framework-language-runtime-probe",
                subject,
                True,
                "scaffold emits only non-standalone source formats; framework runtime remains unverified",
                evidence_status="not_applicable",
            )
        ]

    results: list[Result] = []
    for language in languages:
        results.append(
            _run_framework_language_probe_for_language(
                binary, core, framework, project, env, language
            )
        )
    return results


def _run_framework_language_probe_for_language(
    binary: Path,
    core: str,
    framework: str,
    project: Path,
    env: dict[str, str],
    language: str,
) -> Result:
    """Run one isolated executable-language probe through mgc run.
    Chạy một probe ngôn ngữ độc lập, cách ly, qua mgc run.
    """
    subject = f"{core}/{framework}/{language}"
    if language == "Objective-C" and sys.platform != "darwin":
        return Result(
            "framework-language-runtime-probe",
            subject,
            True,
            "Objective-C runtime probe is applicable only on macOS",
            evidence_status="not_applicable",
        )
    if language == "Objective-C++" and sys.platform != "darwin":
        return Result(
            "framework-language-runtime-probe",
            subject,
            True,
            "Objective-C++ runtime probe is applicable only on macOS",
            evidence_status="not_applicable",
        )
    if language in NON_STANDALONE_RUNTIME_FORMATS:
        return Result(
            "framework-language-runtime-probe",
            subject,
            True,
            f"{language} requires its framework/toolchain lifecycle rather than a standalone language runtime",
            evidence_status="not_applicable",
        )

    profile = language_runtime_profile(language)
    if profile is None:
        return Result(
            "framework-language-runtime-probe",
            subject,
            True,
            f"no standalone runtime probe is defined for {language}; framework runtime remains unverified",
            evidence_status="unverified",
        )

    executable, filename, source, commands = profile
    # Swift provisioning may shadow clang; carry the declared Apple tool and SDK
    # into the MGC script because the executor deliberately starts with a clean env.
    # Swift có thể che clang trong PATH; truyền compiler/SDK Apple đã khai báo vào
    # script MGC vì executor chủ động làm sạch env của process con.
    if sys.platform == "darwin" and language in {"C", "C++", "Objective-C", "Objective-C++"}:
        compiler_key = "CXX" if language in {"C++", "Objective-C++"} else "CC"
        configured_compiler = env.get(compiler_key)
        if configured_compiler:
            commands = (shlex.quote(configured_compiler) + commands[0][len(executable):], *commands[1:])
            executable = configured_compiler
        sdk_root = env.get("SDKROOT")
        if sdk_root:
            commands = (f"SDKROOT={shlex.quote(sdk_root)} {commands[0]}", *commands[1:])
    search_path = env.get("PATH") or env.get("Path")
    if shutil.which(executable, path=search_path) is None:
        return Result(
            "framework-language-runtime-probe",
            subject,
            True,
            f"{language} runtime executable {executable!r} is missing; framework runtime remains unverified",
            evidence_status="unverified",
        )

    if language in {"Java", "Kotlin"}:
        java = run_command(
            ["java", "-version"],
            cwd=project,
            env=env,
            timeout_seconds=30,
        )
        version_match = re.search(r'(?i)(?:version\s+)?"?(\d+)(?:\.(\d+))?', java.stdout)
        if java.returncode != 0 or version_match is None:
            return Result(
                "framework-language-runtime-probe",
                subject,
                True,
                f"{language} tool is present but no usable Java runtime was detected; framework runtime remains unverified: {java.stdout[-600:]}",
                java.returncode,
                "unverified",
            )
        major = int(version_match.group(1))
        if major == 1 and version_match.group(2):
            major = int(version_match.group(2))
        minimum = 11 if language == "Java" else 8
        if major < minimum:
            return Result(
                "framework-language-runtime-probe",
                subject,
                True,
                f"{language} probe requires Java {minimum}+; detected Java {major}; framework runtime remains unverified",
                evidence_status="unverified",
            )

    if language == "C#":
        sdks = run_command(
            ["dotnet", "--list-sdks"],
            cwd=project,
            env=env,
            timeout_seconds=30,
        )
        if sdks.returncode != 0 or not re.search(r"(?m)^10\.", sdks.stdout):
            return Result(
                "framework-language-runtime-probe",
                subject,
                True,
                f"C# probe requires an installed .NET 10 SDK; framework runtime remains unverified: {sdks.stdout[-600:]}",
                sdks.returncode,
                "unverified",
            )

    marker = f"MGC_STRESS_LANGUAGE_RUNTIME:{language}"
    try:
        fixture_root = Path(env.get("MGC_DATA_DIR", project.parent))
        language_key = language.lower().replace("+", "p").replace("#", "sharp")
        probe_project = fixture_root / "language-probes" / core / framework / language_key
        probe_project.mkdir(parents=True, exist_ok=True)
        for config_name in ("mgc.toml", ".mgc.core"):
            source_config = project / config_name
            if source_config.is_file():
                shutil.copy2(source_config, probe_project / config_name)
        if not (probe_project / "mgc.toml").is_file():
            return Result(
                "framework-language-runtime-probe",
                subject,
                False,
                "generated scaffold has no mgc.toml to identify the runtime probe project",
                evidence_status="unverified",
            )

        if language in {"TypeScript", "JavaScript"}:
            (probe_project / "package.json").write_text(
                '{"name":"mgc-stress-language-probe","dependencies":{},"devDependencies":{}}\n',
                encoding="utf-8",
            )
        elif language == "Python":
            (probe_project / "pyproject.toml").write_text(
                '[project]\nname = "mgc-stress-language-probe"\n'
                'version = "0.0.0"\nrequires-python = ">=3.9"\n'
                "dependencies = []\n",
                encoding="utf-8",
            )
        elif language == "Go":
            (probe_project / "go.mod").write_text(
                "module example.com/mgc-stress-language-probe\n\ngo 1.21\n",
                encoding="utf-8",
            )
        elif language == "Dart":
            (probe_project / "pubspec.yaml").write_text(
                "name: mgc_stress_language_probe\nversion: 0.0.0\n",
                encoding="utf-8",
            )
        elif language == "Swift":
            (probe_project / "Package.swift").write_text(
                "// swift-tools-version: 5.9\n",
                encoding="utf-8",
            )

        probe_path = probe_project / filename
        probe_path.parent.mkdir(parents=True, exist_ok=True)
        probe_path.write_text(source, encoding="utf-8")
        if language in {"C", "C++", "Objective-C", "Objective-C++"}:
            compiler_temp = probe_project / ".mgc-compiler-tmp"
            compiler_temp.mkdir(parents=True, exist_ok=True)
            commands = (
                f"TMPDIR={shlex.quote(str(compiler_temp))} {commands[0]}",
                *commands[1:],
            )
        if language == "C#":
            (probe_project / ".mgc-stress-dotnet/MgcStressProbe.csproj").write_text(
                "<Project Sdk=\"Microsoft.NET.Sdk\">\n"
                "  <PropertyGroup>\n"
                "    <OutputType>Exe</OutputType>\n"
                "    <TargetFramework>net10.0</TargetFramework>\n"
                "    <ImplicitUsings>enable</ImplicitUsings>\n"
                "    <Nullable>enable</Nullable>\n"
                "  </PropertyGroup>\n"
                "</Project>\n",
                encoding="utf-8",
            )
            (probe_project / "NuGet.Config").write_text(
                "<configuration><packageSources><clear /></packageSources></configuration>\n",
                encoding="utf-8",
            )
        if language == "GDScript":
            (probe_project / ".mgc-stress-godot/project.godot").write_text(
                "config_version=5\n\n[application]\nconfig/name=\"MGC Stress Probe\"\n",
                encoding="utf-8",
            )
        if language == "Rust":
            rust_source = probe_path.parent / "src" / "main.rs"
            rust_source.parent.mkdir(parents=True, exist_ok=True)
            rust_source.write_text(
                f'fn main() {{ println!("{marker}"); }}\n', encoding="utf-8"
            )
            (probe_path.parent / "Cargo.lock").write_text(
                'version = 4\n\n[[package]]\n'
                'name = "mgc-stress-runtime-probe"\nversion = "0.0.0"\n',
                encoding="utf-8",
            )
        if language == "Java":
            (probe_project / ".mgc-java-tmp").mkdir(parents=True, exist_ok=True)
        if language == "Kotlin":
            (probe_project / ".mgc-java-tmp").mkdir(parents=True, exist_ok=True)
        probe_env = env.copy()
        if language == "Go":
            # Explicit cache survives MGC's clean child environment.
            # Cache tường minh nằm trong fixture, không ghi vào cache máy thật.
            go_cache = probe_project / ".mgc-go-build-cache"
            go_cache.mkdir(parents=True, exist_ok=True)
            commands = tuple(
                f"GOCACHE={shlex.quote(str(go_cache))} {command}"
                for command in commands
            )
            probe_env.update(
                {"GOPROXY": "off", "GOSUMDB": "off", "GOTOOLCHAIN": "local"}
            )
        if language == "Swift":
            swift_cache = probe_project / ".mgc-swift-module-cache"
            swift_sdk_cache = probe_project / ".mgc-swift-sdk-module-cache"
            swift_cache.mkdir(parents=True, exist_ok=True)
            swift_sdk_cache.mkdir(parents=True, exist_ok=True)
            probe_env["CLANG_MODULE_CACHE_PATH"] = str(swift_cache)
            probe_env["SWIFT_MODULECACHE_PATH"] = str(swift_cache)
        if language == "C#":
            restore = run_command(
                [
                    "dotnet",
                    "restore",
                    ".mgc-stress-dotnet/MgcStressProbe.csproj",
                    "--configfile",
                    "NuGet.Config",
                    "--ignore-failed-sources",
                ],
                cwd=probe_project,
                env=probe_env,
                timeout_seconds=120,
            )
            if restore.returncode != 0:
                return Result(
                    "framework-language-runtime-probe",
                    subject,
                    False,
                    f"package-free .NET fixture restore failed: {restore.stdout[-2200:]}",
                    restore.returncode,
                    "unverified",
                )
            build = run_command(
                [
                    "dotnet",
                    "build",
                    ".mgc-stress-dotnet/MgcStressProbe.csproj",
                    "--no-restore",
                ],
                cwd=probe_project,
                env=probe_env,
                timeout_seconds=120,
            )
            if build.returncode != 0:
                return Result(
                    "framework-language-runtime-probe",
                    subject,
                    False,
                    f"offline .NET fixture build failed: {build.stdout[-2200:]}",
                    build.returncode,
                    "unverified",
                )

        outputs: list[str] = []
        completed: subprocess.CompletedProcess[str] | None = None
        for step, command in enumerate(commands, start=1):
            script_name = f"mgc_language_runtime_probe_{step}"
            upsert_script_entries(
                probe_project / "mgc.toml",
                {script_name: command},
            )
            completed = run_command(
                [str(binary), "run", "--core", core, script_name],
                cwd=probe_project,
                env=probe_env,
                timeout_seconds=120,
            )
            outputs.append(completed.stdout)
            if completed.returncode != 0:
                break
        output = "\n".join(outputs)
        returncode = completed.returncode if completed is not None else 1
        passed = returncode == 0 and (
            language == "Terraform HCL" or marker in output
        )
        failure_detail = (
            output
            if len(output) <= 3000
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
            passed,
            detail,
            returncode,
            "verified" if passed else "unverified",
        )
    except (OSError, shutil.Error, subprocess.TimeoutExpired, UnicodeError) as error:
        return Result(
            "framework-language-runtime-probe",
            subject,
            False,
            f"{language} runtime probe failed: {error}",
            evidence_status="unverified",
        )


def prepare_runtime_smoke_manifest(project: Path, core: str, framework: str) -> str | None:
    """Remove declared dependencies before testing command orchestration alone.
    Bỏ dependency khai báo để kiểm riêng orchestration của lệnh.
    """
    manifest_names = (
        "package.json",
        "Cargo.toml",
        "pyproject.toml",
        "go.mod",
        "pom.xml",
        "pubspec.yaml",
        "Package.swift",
    )
    has_manifest = any((project / name).is_file() for name in manifest_names)
    has_manifest = has_manifest or next(project.glob("*.csproj"), None) is not None
    if not has_manifest:
        return None
    return prepare_empty_dependency_manifest(project, core, framework)


def prepare_empty_dependency_manifest(project: Path, core: str, framework: str) -> str:
    """Keep framework identity while removing network dependencies for install smoke.
    Giữ identity framework nhưng bỏ dependency mạng cho smoke install.
    """
    package_json = project / "package.json"
    if package_json.is_file():
        package = json.loads(package_json.read_text(encoding="utf-8"))
        if not isinstance(package, dict):
            raise ValueError("package.json root is not an object")
        for field in ("dependencies", "devDependencies", "optionalDependencies", "peerDependencies"):
            package[field] = {}
        package_json.write_text(json.dumps(package, indent=2) + "\n", encoding="utf-8")
        return "package.json"

    cargo_toml = project / "Cargo.toml"
    if cargo_toml.is_file():
        name = project.name.replace("_", "-")
        cargo_toml.write_text(
            f'[package]\nname = "{name}"\nversion = "0.1.0"\nedition = "2021"\n',
            encoding="utf-8",
        )
        return "Cargo.toml"

    pyproject = project / "pyproject.toml"
    if pyproject.is_file():
        name = project.name.replace("_", "-")
        pyproject.write_text(
            f'[project]\nname = "{name}"\nversion = "0.1.0"\n'
            'requires-python = ">=3.9"\ndependencies = []\n',
            encoding="utf-8",
        )
        return "pyproject.toml"

    go_mod = project / "go.mod"
    if go_mod.is_file():
        go_mod.write_text(
            f"module example.com/{project.name.lower()}\n\ngo 1.21\n",
            encoding="utf-8",
        )
        return "go.mod"

    pom = project / "pom.xml"
    if pom.is_file():
        pom.write_text(
            "<project><modelVersion>4.0.0</modelVersion><groupId>local</groupId>"
            f"<artifactId>{project.name}</artifactId><version>0.1.0</version></project>\n",
            encoding="utf-8",
        )
        return "pom.xml"

    csproj = next(project.glob("*.csproj"), None)
    if csproj is not None:
        csproj.write_text(
            '<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup>'
            '<TargetFramework>net8.0</TargetFramework></PropertyGroup></Project>\n',
            encoding="utf-8",
        )
        return csproj.name

    pubspec = project / "pubspec.yaml"
    if pubspec.is_file():
        pubspec.write_text(
            f'name: {project.name.lower()}\nversion: 0.1.0\n'
            'environment:\n  sdk: ">=3.0.0 <4.0.0"\ndependencies: {}\n',
            encoding="utf-8",
        )
        return "pubspec.yaml"

    swift_package = project / "Package.swift"
    if swift_package.is_file():
        swift_package.write_text(
            '// swift-tools-version: 5.9\nimport PackageDescription\n'
            f'let package = Package(name: "{project.name}", products: [], dependencies: [], targets: [])\n',
            encoding="utf-8",
        )
        return "Package.swift"

    raise ValueError(f"no supported dependency manifest found for {core}/{framework}")


def run_native_install_contract(
    binary: Path,
    row: dict[str, Any],
    project: Path,
    env: dict[str, str],
) -> tuple[Result | None, bool, str | None]:
    """Run install only where capability data assigns MGC native ownership.
    Chỉ chạy install ở lane mà capability giao quyền cho MGC native.
    """
    core = row["core"]
    framework = row["framework"]
    subject = f"{core}/{framework}"
    owner = row["dependency_ownership"]["install"]["owner"]
    if owner != "mgc-native":
        return (
            Result(
                "framework-install-ownership",
                subject,
                True,
                f"install is declared {owner}; native execution not claimed",
                evidence_status="unsupported",
            ),
            False,
            None,
        )

    try:
        manifest = prepare_empty_dependency_manifest(project, core, framework)
        set_lock_schema(project / "mgc.toml", "3")
        completed = run_command(
            [str(binary), f"install-{core}"],
            cwd=project,
            env=env,
            timeout_seconds=120,
        )
        passed = completed.returncode == 0 and "All dependencies installed" in completed.stdout
        detail = (
            f"native install completed with an empty {manifest} dependency set"
            if passed
            else completed.stdout[-1400:]
        )
        return (
            Result("mgc-install-empty-graph", subject, passed, detail, completed.returncode),
            passed,
            manifest,
        )
    except (OSError, UnicodeError, ValueError, json.JSONDecodeError) as error:
        return Result("mgc-install-empty-graph", subject, False, str(error)), False, None


def run_store_scope_contract(
    binary: Path,
    row: dict[str, Any],
    project: Path,
    env: dict[str, str],
    install_succeeded: bool,
) -> list[Result]:
    """Verify the declared MGC store route and its runtime health evidence.
    Xác minh store route đã khai báo và bằng chứng health lúc chạy.
    """
    core = row["core"]
    framework = row["framework"]
    subject = f"{core}/{framework}"
    install_owner = row["dependency_ownership"]["install"]["owner"]
    store_owner = row["dependency_ownership"]["store"]["owner"]
    if store_owner != "mgc-native":
        status = "not_applicable" if core == "web" and install_owner != "mgc-native" else "unsupported"
        detail = (
            "capability manifest declares no MGC-owned store route for this framework"
        )
        return [
            Result("shared-store-scope", subject, True, detail, evidence_status=status),
            Result("shared-store-runtime", subject, True, detail, evidence_status=status),
        ]
    if not install_succeeded:
        detail = "MGC-owned install did not complete; store health has no runtime evidence"
        return [
            Result("shared-store-scope", subject, False, detail, evidence_status="unverified"),
            Result("shared-store-runtime", subject, False, detail, evidence_status="unverified"),
        ]

    try:
        completed = run_command(
            [
                str(binary),
                "store",
                "doctor",
                "--core",
                core,
                "--framework",
                framework,
                "--json",
            ],
            cwd=project,
            env=env,
            timeout_seconds=45,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        detail = str(error)
        return [
            Result("shared-store-scope", subject, False, detail, evidence_status="unverified"),
            Result("shared-store-runtime", subject, False, detail, evidence_status="unverified"),
        ]

    payload = _parse_json_object(completed.stdout)
    reported_scope = payload.get("scope") if payload else None
    scope_verified = bool(
        payload
        and payload.get("core") == core
        and payload.get("framework") == framework
        and reported_scope in {
            "web-cas-and-generation-integrity",
            "managed-root-path-safety-only",
        }
    )
    scope_result = Result(
        "shared-store-scope",
        subject,
        scope_verified,
        (
            f"doctor selected {reported_scope} for {core}/{framework}"
            if scope_verified
            else completed.stdout[-1200:]
        ),
        completed.returncode,
        "verified" if scope_verified else "unverified",
    )

    runtime_evidence_status = "unverified"
    runtime_check_passed = False
    if reported_scope == "web-cas-and-generation-integrity":
        runtime_checks_ok = all(
            payload.get(key) is True
            for key in ("db_ok", "cas_root_ok", "generation_state_ok")
        ) if payload else False
        failed_checks = [
            key
            for key in ("db_ok", "cas_root_ok", "generation_state_ok")
            if not payload or payload.get(key) is not True
        ]
        runtime_detail = (
            "Web CAS database, CAS root and generation invariants are healthy"
            if runtime_checks_ok
            else f"Web store health check failed: {', '.join(failed_checks)}"
        )
        runtime_check_passed = runtime_checks_ok
        if scope_verified:
            runtime_evidence_status = "verified"
    elif reported_scope == "managed-root-path-safety-only":
        roots = payload.get("roots") if payload else None
        roots_shape_ok = bool(
            payload
            and payload.get("integrity_checked") is False
            and isinstance(roots, list)
            and roots
            and all(
                isinstance(root, dict)
                and isinstance(root.get("name"), str)
                and isinstance(root.get("exists"), bool)
                and isinstance(root.get("safe"), bool)
                for root in roots
            )
        )
        roots_safe = bool(roots_shape_ok and all(root["safe"] for root in roots))
        # Root path safety proves route selection and storage-boundary health,
        # not that a non-empty dependency install works or stored bytes are
        # intact. Keep runtime unverified until a package-bearing fixture
        # exercises this lane end to end.
        if roots_shape_ok and not roots_safe:
            runtime_evidence_status = "verified"
            runtime_detail = "a selected MGC shared root failed path-safety validation"
        elif roots_shape_ok and roots_safe:
            # The doctor's declared scope passed; the broader package runtime
            # still has no non-empty install evidence. Keep that distinction
            # in evidence_status instead of treating missing proof as a health
            # failure in the normal CI contract.
            runtime_check_passed = True
            runtime_detail = (
                "all selected roots passed path-safety checks, but no package-bearing install "
                "or package-byte integrity check ran; shared-store runtime remains unverified"
            )
        else:
            runtime_detail = "managed-root status did not include valid root evidence"
    else:
        runtime_detail = completed.stdout[-1200:]
    runtime_ok = completed.returncode == 0 and runtime_check_passed and scope_verified
    runtime_result = Result(
        "shared-store-runtime",
        subject,
        runtime_ok,
        runtime_detail if scope_verified else completed.stdout[-1200:],
        completed.returncode,
        runtime_evidence_status if scope_verified else "unverified",
    )
    return [scope_result, runtime_result]


def run_cli_contracts(binary: Path, root: Path, env: dict[str, str]) -> list[Result]:
    """Check every discoverable command/subcommand help path and bad options.
    Kiểm tra help của mọi lệnh/subcommand được công bố và option sai.
    """
    results: list[Result] = []
    try:
        root_help = run_command(
            [str(binary), "--help"], cwd=root, env=env, timeout_seconds=10
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        root_help = None
        root_detail = str(error)
        root_returncode = None
    else:
        root_detail = root_help.stdout[-1000:] or "root help returned no Usage section"
        root_returncode = root_help.returncode

    if root_help is None or root_help.returncode != 0 or "Usage:" not in root_help.stdout:
        results.append(
            Result(
                "cli-help-inventory",
                "mgc --help",
                False,
                root_detail,
                root_returncode,
            )
        )
        command_paths: list[tuple[str, ...]] = []
    else:
        command_paths = [(name,) for name in parse_help_commands(root_help.stdout)]
        if not command_paths:
            results.append(
                Result(
                    "cli-help-inventory",
                    "mgc --help command inventory",
                    False,
                    "root help has no discoverable Commands section",
                    root_help.returncode,
                )
            )

    visited: set[tuple[str, ...]] = set()
    while command_paths:
        path = command_paths.pop(0)
        if path in visited:
            continue
        if len(visited) >= MAX_CLI_HELP_COMMANDS:
            results.append(
                Result(
                    "cli-help-inventory",
                    "command tree limit",
                    False,
                    f"more than {MAX_CLI_HELP_COMMANDS} unique command paths were advertised",
                )
            )
            break
        visited.add(path)
        is_help_command = path[-1] == "help"
        argv = [str(binary), *path]
        if not is_help_command:
            argv.append("--help")
        label = "mgc " + " ".join(path)
        if not is_help_command:
            label += " --help"
        try:
            completed = run_command(
                argv, cwd=root, env=env, timeout_seconds=10
            )
            passed = completed.returncode == 0 and "Usage:" in completed.stdout
            detail = "help returned successfully" if passed else completed.stdout[-1000:]
            results.append(Result("cli-command-help", label, passed, detail, completed.returncode))
            if passed and not is_help_command:
                command_paths.extend(
                    path + (name,) for name in parse_help_commands(completed.stdout)
                )
        except (OSError, subprocess.TimeoutExpired) as error:
            results.append(Result("cli-command-help", label, False, str(error)))

        invalid_argv = [str(binary), *path, "--definitely-invalid-mgc-option"]
        invalid_label = "mgc " + " ".join(path) + " rejects unknown option"
        try:
            invalid = run_command(
                invalid_argv, cwd=root, env=env, timeout_seconds=10
            )
            output_lower = invalid.stdout.lower()
            passed = (
                invalid.returncode != 0
                and "--definitely-invalid-mgc-option" in invalid.stdout
                and any(
                    marker in output_lower
                    for marker in ("unexpected argument", "unrecognized subcommand")
                )
            )
            results.append(
                Result(
                    "security-cli-command",
                    invalid_label,
                    passed,
                    "unknown option rejected before command execution"
                    if passed
                    else invalid.stdout[-1000:],
                    invalid.returncode,
                )
            )
        except (OSError, subprocess.TimeoutExpired) as error:
            results.append(Result("security-cli-command", invalid_label, False, str(error)))

    try:
        completed = run_command(
            [str(binary), "--definitely-invalid-mgc-option"],
            cwd=root,
            env=env,
            timeout_seconds=10,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        results.append(
            Result("security-cli", "unknown global option is rejected", False, str(error))
        )
    else:
        passed = completed.returncode != 0 and "Usage:" in completed.stdout
        results.append(
            Result(
                "security-cli",
                "unknown global option is rejected",
                passed,
                "invalid option rejected" if passed else completed.stdout[-1000:],
                completed.returncode,
            )
        )
    return results


def parse_help_commands(output: str) -> list[str]:
    """Read clap's immediate child commands from a help section.
    Đọc các lệnh con trực tiếp từ phần help do clap sinh.
    """
    lines = output.splitlines()
    try:
        start = next(
            index
            for index, line in enumerate(lines)
            if line.strip() == "Commands:"
        ) + 1
    except StopIteration:
        return []

    commands: list[str] = []
    for line in lines[start:]:
        if not line.strip():
            break
        match = re.match(r"^  ([a-z][a-z0-9-]*)(?:\s{2,}.*)?$", line)
        if match:
            commands.append(match.group(1))
    return commands


def create_frameworks(
    binary: Path,
    rows: list[dict[str, Any]],
    root: Path,
    env: dict[str, str],
    timeout_seconds: int,
) -> tuple[list[Result], list[tuple[str, str, Path]]]:
    """Create every advertised framework in its own isolated directory.
    Tạo mọi framework được công bố trong thư mục cách ly riêng.
    """
    results: list[Result] = []
    successful_projects: list[tuple[str, str, Path]] = []
    for row in rows:
        core = row["core"]
        framework = row["framework"]
        project_name = project_slug(core, framework)
        core_root = root / "projects" / core
        core_root.mkdir(parents=True, exist_ok=True)
        argv = [str(binary), f"create-{core}", framework, project_name]
        if core == "web":
            argv.append("--yes")
        try:
            completed = run_command(
                argv,
                cwd=core_root,
                env=env,
                timeout_seconds=timeout_seconds,
            )
            project = core_root / project_name
            detail = validate_scaffold(project, core, framework) if completed.returncode == 0 else None
            passed = completed.returncode == 0 and detail is None
            subject = f"{core}/{framework}"
            if passed:
                successful_projects.append((core, framework, project))
                summary = f"created {project_name} ({row['status']})"
                language_files, suffix_counts = scaffold_language_files(project, core)
                language_detail = json.dumps(
                    {
                        "languages_or_source_formats": sorted(language_files),
                        "source_files_by_language_or_format": language_files,
                        "source_suffix_counts": suffix_counts,
                        "runtime_verified": False,
                    },
                    sort_keys=True,
                )
                language_passed = bool(language_files)
                results.append(
                    Result(
                        "framework-language-scaffold",
                        subject,
                        language_passed,
                        language_detail
                        if language_passed
                        else "scaffold passed but no recognized source-language file was found",
                        completed.returncode,
                        "verified" if language_passed else "unverified",
                    )
                )
            else:
                summary = detail or f"create command exited {completed.returncode}: {completed.stdout[-1400:]}"
            results.append(Result("framework-scaffold", subject, passed, summary, completed.returncode))
        except subprocess.TimeoutExpired as error:
            results.append(Result("framework-scaffold", f"{core}/{framework}", False, f"timed out after {timeout_seconds}s: {error}"))
        except (OSError, UnicodeError) as error:
            results.append(Result("framework-scaffold", f"{core}/{framework}", False, str(error)))
    return results, successful_projects


def scaffold_language_files(project: Path, core: str) -> tuple[dict[str, int], dict[str, int]]:
    """Inventory source files emitted by one scaffold, without executing them.
    Lập danh mục file nguồn do scaffold tạo, không thực thi code.
    """
    language_counts: Counter[str] = Counter()
    suffix_counts: Counter[str] = Counter()
    ignored_directories = {".git", ".magicore", ".mgc", "node_modules", "target", "dist"}
    for path in project.rglob("*"):
        relative_parts = path.relative_to(project).parts
        if not path.is_file() or any(
            part in ignored_directories or part.startswith(".mgc")
            for part in relative_parts
        ):
            continue
        relative = path.relative_to(project).as_posix().lower()
        suffix = path.suffix.lower()
        language = SOURCE_LANGUAGE_BY_SUFFIX.get(suffix)
        if core == "cicd" and (
            relative in AUTOMATION_PATHS
            or (relative.startswith(".github/workflows/") and suffix in {".yaml", ".yml"})
        ):
            language = "CI workflow YAML"
        elif core == "hardware" and path.name in {"bench.json", "optimizer.json"}:
            language = "JSON hardware descriptor"
        if language is None:
            continue
        language_counts[language] += 1
        suffix_counts[suffix or "<no extension>"] += 1
    return dict(sorted(language_counts.items())), dict(sorted(suffix_counts.items()))


def run_project_dimensions(
    binary: Path,
    projects: list[tuple[str, str, Path]],
    rows: list[dict[str, Any]],
    root: Path,
    env: dict[str, str],
) -> list[Result]:
    """Exercise runtime scripts, owned installs, store scope, and quality dimensions.
    Chạy script runtime, install được MGC sở hữu, scope store và quality dimensions.
    """
    results: list[Result] = []
    rows_by_identity = {(row["core"], row["framework"]): row for row in rows}
    for core, framework, project in sorted(projects):
        row = rows_by_identity.get((core, framework))
        if row is None:
            results.append(
                Result(
                    "framework-capability-link",
                    f"{core}/{framework}",
                    False,
                    "scaffold has no matching capability row",
                )
            )
            continue

        # Run the declared native install before copying quality fixtures so
        # audit sees the lock state produced by this exact scaffold. Native
        # routes rewrite dependencies to an empty graph for this matrix.
        # (Cài theo owner native trước khi copy fixture quality để audit nhìn
        # đúng lock do scaffold tạo; lane native dùng graph rỗng có chủ đích.)
        install_result, install_succeeded, empty_dependency_manifest = run_native_install_contract(
            binary, row, project, env
        )
        if install_result is not None:
            results.append(install_result)
        empty_graph_installed = install_succeeded and empty_dependency_manifest is not None
        native_install_owned = row["dependency_ownership"]["install"]["owner"] == "mgc-native"

        # Each quality probe receives a fresh copy so neither synthetic
        # lifecycle edits nor optimizer output can contaminate another check.
        # (Mỗi probe quality dùng bản sao riêng để script giả và output
        # optimizer không làm nhiễm kết quả của chiều kiểm tra khác.)
        commands = (
            ("optimizer", ["optimizer", "--core", core]),
            ("cache", ["cache", "status", "--target", "all", "--core", core]),
            ("security-audit", ["audit", "--core", core, "--format", "json"]),
        )
        for category, args in commands:
            quality_project = (
                root / "quality-fixtures" / category / core / project.name
            )
            try:
                quality_project.parent.mkdir(parents=True, exist_ok=True)
                shutil.copytree(project, quality_project, symlinks=True)
                completed = run_command(
                    [str(binary), *args],
                    cwd=quality_project,
                    env=env,
                    timeout_seconds=90,
                )
                passed, evidence_status, detail = assess_dimension(
                    category,
                    completed,
                    empty_graph_installed=empty_graph_installed,
                    native_install_owned=native_install_owned,
                    optimizer_no_runtime_is_not_applicable=True,
                )
                results.append(
                    Result(
                        category,
                        f"{core}/{framework}",
                        passed,
                        detail,
                        completed.returncode,
                        evidence_status,
                    )
                )
            except (OSError, shutil.Error, subprocess.TimeoutExpired) as error:
                results.append(Result(category, f"{core}/{framework}", False, str(error)))

        results.extend(
            run_framework_language_runtime_probe(
                binary, core, framework, project, env
            )
        )
        results.extend(run_framework_script_contracts(binary, core, framework, project, env))
        results.extend(run_store_scope_contract(binary, row, project, env, install_succeeded))
        results.extend(
            run_lockfile_contracts(
                binary,
                row,
                project,
                env,
                install_succeeded,
            )
        )

    results.extend(run_shared_store_check(binary, root, env))
    return results


def run_shared_store_check(binary: Path, root: Path, env: dict[str, str]) -> list[Result]:
    """Initialize and inspect an isolated empty web store without network dependencies.
    Khởi tạo và kiểm tra web store tách biệt, không cần tải dependency qua mạng.
    """
    project = root / "shared-store-project"
    project.mkdir(parents=True, exist_ok=True)
    (project / ".mgc.core").write_text("web\n", encoding="utf-8")
    (project / "mgc.toml").write_text(
        'name = "shared-store-stress"\nversion = "0.1.0"\necosystem = "web"\nframeworks = ["vanilla"]\n',
        encoding="utf-8",
    )
    (project / "package.json").write_text(
        '{"name":"shared-store-stress","version":"0.1.0","dependencies":{}}\n',
        encoding="utf-8",
    )
    try:
        install = run_command(
            [str(binary), "install-web"], cwd=project, env=env, timeout_seconds=90
        )
        if install.returncode != 0:
            return [Result("shared-store", "web", False, f"empty install failed: {install.stdout[-1200:]}", install.returncode)]
        completed = run_command(
            [str(binary), "store", "doctor", "--core", "web", "--json"],
            cwd=project,
            env=env,
            timeout_seconds=45,
        )
        payload = _parse_json_object(completed.stdout)
        healthy = bool(payload and payload.get("healthy") is True)
        passed = completed.returncode == 0 and healthy
        detail = "web shared store reports healthy after install" if passed else completed.stdout[-1200:]
        return [Result("shared-store", "web", passed, detail, completed.returncode)]
    except (OSError, subprocess.TimeoutExpired) as error:
        return [Result("shared-store", "web", False, str(error))]


def _parse_json_object(output: str) -> dict[str, Any] | None:
    """Find a JSON object after optional CLI warnings.
    Tìm JSON object sau các warning tùy chọn của CLI.
    """
    for index, character in enumerate(output):
        if character != "{":
            continue
        try:
            payload, _end = json.JSONDecoder().raw_decode(output[index:])
        except json.JSONDecodeError:
            continue
        if isinstance(payload, dict):
            return payload
    return None


def _is_json_object(output: str) -> bool:
    return _parse_json_object(output) is not None


def assess_dimension(
    category: str,
    completed: subprocess.CompletedProcess[str],
    *,
    empty_graph_installed: bool = False,
    native_install_owned: bool = True,
    optimizer_no_runtime_is_not_applicable: bool = False,
) -> tuple[bool, str, str]:
    """Separate verified capability from an expected but unverified lane.
    Tách năng lực đã xác minh khỏi lane chưa có bằng chứng, nhưng từ chối đúng.
    """
    if category == "optimizer":
        if completed.returncode == 0:
            return True, "verified", "optimizer completed"
        if "No runtime detected for" in completed.stdout and "nothing was optimized" in completed.stdout:
            if optimizer_no_runtime_is_not_applicable:
                return True, "not_applicable", "no optimizer runtime is registered for this project; no optimizer capability is claimed for this lane"
            return True, "unverified", "no optimizer runtime is registered for this project; no optimization claim made"
        return False, "failed", completed.stdout[-1400:]

    if category == "cache":
        return (
            completed.returncode == 0,
            "verified" if completed.returncode == 0 else "failed",
            "cache status completed" if completed.returncode == 0 else completed.stdout[-1400:],
        )

    if category == "security-audit":
        payload = _parse_json_object(completed.stdout)
        if payload is None:
            if completed.returncode != 0 and "native Cargo dependency resolution" in completed.stdout:
                if empty_graph_installed or not native_install_owned:
                    return True, "not_applicable", "this fixture has no MGC-owned non-empty Cargo graph; feature auditing is not qualified here"
                return True, "unverified", "the native resolver cannot model this dependency's feature semantics"
            if completed.returncode != 0 and any(
                phrase in completed.stdout.lower()
                for phrase in ("cargo.lock", "mgc.lock", "lockfile is missing", "no lockfile")
            ):
                if empty_graph_installed or not native_install_owned:
                    return True, "not_applicable", "this matrix fixture has no MGC-owned resolved dependency graph to audit"
                return True, "unverified", "audit refused to infer dependencies because a lockfile is missing"
            return False, "failed", completed.stdout[-1400:]
        scanner_status = payload.get("scanner_status")
        vulnerabilities = payload.get("vulnerability_count", 0)
        packages_audited = payload.get("packages_audited", 0)
        if isinstance(vulnerabilities, int) and vulnerabilities > 0:
            return False, "failed", f"audit found {vulnerabilities} finding(s); see command output"
        if completed.returncode == 0 and scanner_status == "available":
            if packages_audited == 0:
                if empty_graph_installed or not native_install_owned:
                    return True, "not_applicable", "no resolved packages in this matrix fixture; non-empty audit behavior is covered by audit E2E"
                return True, "unverified", "scanner is available, but this scaffold has no resolved packages to scan"
            return True, "verified", f"audit scanned {packages_audited} package(s)"
        if scanner_status in {"partial", "failed", "unsupported_ecosystem", "unavailable"}:
            reasons = payload.get("skipped_reasons", [])
            reason = "; ".join(str(item) for item in reasons) or "no resolved lockfile input"
            if empty_graph_installed or not native_install_owned:
                return True, "not_applicable", "no MGC-owned resolved dependency graph in this matrix fixture; non-empty audit behavior is covered by audit E2E"
            return True, "unverified", f"strict audit refused an incomplete scan: {reason}"
        return False, "failed", completed.stdout[-1400:]

    return False, "failed", f"unknown stress dimension: {category}"


def write_report(
    path: Path,
    version: str,
    results: list[Result],
    rows: list[dict[str, Any]],
    *,
    universal_gate_enforced: bool = False,
    support_gaps: list[str] | None = None,
    source_provenance: dict[str, Any] | None = None,
) -> None:
    """Write machine-readable run evidence.
    Ghi bằng chứng lần chạy ở định dạng máy đọc.
    """
    runtime_verified = framework_runtime_verified(results, rows)
    payload = {
        "schema_version": 7,
        "generated_at": dt.datetime.now(dt.timezone.utc).isoformat(),
        "mgc_version": version,
        "platform": platform.platform(),
        "source_provenance": (
            source_provenance
            if source_provenance is not None
            else {
                "git_commit": None,
                "working_tree_clean": None,
                "binary_sha256": None,
            }
        ),
        "framework_inventory": len(rows),
        "evidence_contract": {
            "framework_runtime_verified": runtime_verified,
            "run_test": "synthetic Python sentinel is orchestration-only; universal proof requires per-framework CLI runtime evidence",
            "language_runtime": "universal proof requires one verified probe per standalone source language emitted by each scaffold; framework-only source formats rely on framework lifecycle evidence",
            "native_install": "dependency manifests are rewritten to empty graphs before install",
            "lockfile": "V3/V4 checks verify empty-graph write and frozen replay only",
            "framework_lifecycle": "real native lifecycle evidence is imported from an E2E sidecar for the 18 listed web lanes only; absent or incomplete sidecars do not establish support",
            "language_inventory": "generated source extensions only; no language runtime or framework command is executed",
        },
        "framework_lifecycle_evidence": framework_lifecycle_evidence_summary(results),
        "source_language_inventory": source_language_inventory_summary(results),
        "universal_support_gate": {
            "enforced": universal_gate_enforced,
            "status": (
                "not-evaluated"
                if support_gaps is None
                else "ready"
                if not support_gaps
                else "blocked"
            ),
            "blocker_count": None if support_gaps is None else len(support_gaps),
            "blockers": [] if support_gaps is None else support_gaps,
            "required_framework_categories": list(UNIVERSAL_FRAMEWORK_CATEGORIES),
            "required_language_runtime_evidence": "one verified runtime/parser probe per standalone language emitted by each framework scaffold",
            "required_global_categories": list(UNIVERSAL_GLOBAL_CATEGORIES),
            "dependency_operations_require_native": list(DEPENDENCY_OPERATIONS),
        },
        "dependency_ownership_declaration": dependency_ownership_summary(rows),
        "passed": sum(result.passed for result in results),
        "failed": sum(not result.passed for result in results),
        "verified_evidence": sum(
            result.passed and result.evidence_status == "verified" for result in results
        ),
        "unverified_evidence": sum(result.evidence_status == "unverified" for result in results),
        "unsupported_evidence": sum(result.evidence_status == "unsupported" for result in results),
        "not_applicable_evidence": sum(result.evidence_status == "not_applicable" for result in results),
        "results": [asdict(result) for result in results],
    }
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def read_web_lifecycle_evidence(path: str | Path | None, binary: Path) -> list[Result]:
    """Import complete, binary-bound evidence from native web lifecycle E2Es.
    Nạp bằng chứng E2E web đầy đủ và gắn với đúng binary đang được stress-test.
    """
    if path is None or not str(path).strip():
        return []

    sidecar = Path(path).expanduser()
    errors: list[str] = []
    # A path can be reused for a replacement executable; bind evidence to its bytes.
    # Path có thể giữ nguyên khi thay executable; evidence phải khớp byte hiện tại.
    binary_digest = hashlib.sha256()
    try:
        with binary.open("rb") as executable:
            while chunk := executable.read(1024 * 1024):
                binary_digest.update(chunk)
        expected_binary_sha256 = binary_digest.hexdigest()
    except OSError:
        expected_binary_sha256 = None
        errors.append("cannot hash the CLI binary")
    records: dict[tuple[str, str], list[dict[str, Any]]] = defaultdict(list)
    record_count = 0
    digest: str | None = None
    try:
        with sidecar.open("rb") as evidence_file:
            raw = evidence_file.read(MAX_WEB_LIFECYCLE_EVIDENCE_BYTES + 1)
            over_limit = len(raw) > MAX_WEB_LIFECYCLE_EVIDENCE_BYTES
            if not over_limit:
                over_limit = bool(evidence_file.read(1))
        if over_limit:
            errors.append("sidecar exceeds the 1 MiB evidence limit")
        else:
            digest = hashlib.sha256(raw).hexdigest()
            lines = raw.decode("utf-8").splitlines()
            if not lines:
                errors.append("sidecar is empty")
            for line_number, line in enumerate(lines, start=1):
                record_count += 1
                try:
                    value = json.loads(line)
                except json.JSONDecodeError:
                    errors.append(f"line {line_number}: malformed JSON")
                    continue
                if not isinstance(value, dict):
                    errors.append(f"line {line_number}: record must be a JSON object")
                    continue

                framework = value.get("framework")
                category = value.get("category")
                if (
                    not isinstance(framework, str)
                    or framework not in WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK
                ):
                    errors.append(f"line {line_number}: unknown framework")
                    continue
                if category not in FRAMEWORK_RUNTIME_CATEGORIES:
                    errors.append(f"line {line_number}: unknown lifecycle category")
                    continue

                expected_suite = WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK[framework]
                if value.get("schema_version") != 1 or isinstance(value.get("schema_version"), bool):
                    errors.append(f"line {line_number}: unsupported evidence schema")
                    continue
                if value.get("core") != "web" or value.get("suite") != expected_suite:
                    errors.append(f"line {line_number}: core or suite does not match the framework")
                    continue
                passed = value.get("passed")
                returncode = value.get("returncode")
                detail = value.get("detail")
                if (
                    not isinstance(passed, bool)
                    or not isinstance(returncode, int)
                    or isinstance(returncode, bool)
                    or not isinstance(detail, str)
                    or not detail.strip()
                ):
                    errors.append(f"line {line_number}: malformed result fields")
                    continue
                if passed != (returncode == 0):
                    errors.append(f"line {line_number}: pass status conflicts with exit code")
                    continue
                recorded_binary = value.get("binary_path")
                if not isinstance(recorded_binary, str) or not recorded_binary.strip():
                    errors.append(f"line {line_number}: binary path is missing")
                    continue
                try:
                    same_binary = Path(recorded_binary).expanduser().resolve() == binary.resolve()
                except (OSError, RuntimeError):
                    same_binary = False
                if not same_binary:
                    errors.append(f"line {line_number}: evidence belongs to a different CLI binary")
                    continue
                if value.get("binary_sha256") != expected_binary_sha256 or expected_binary_sha256 is None:
                    errors.append(f"line {line_number}: CLI binary digest is missing or mismatched")
                    continue
                records[(framework, category)].append(value)
    except (OSError, UnicodeError) as error:
        errors.append(f"cannot read sidecar: {error.__class__.__name__}")

    imported: list[Result] = []
    expected_keys = {
        (framework, category)
        for framework in WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK
        for category in FRAMEWORK_RUNTIME_CATEGORIES
    }
    for framework, category in sorted(expected_keys):
        matching = records.get((framework, category), [])
        subject = f"web/{framework}"
        if not matching:
            errors.append(f"missing {category} evidence for {subject}")
            imported.append(
                Result(category, subject, False, "missing lifecycle evidence record", None, "failed")
            )
        elif len(matching) != 1:
            errors.append(f"duplicate {category} evidence for {subject}")
            imported.append(
                Result(category, subject, False, "duplicate lifecycle evidence records", None, "failed")
            )
        else:
            record = matching[0]
            passed = record["passed"]
            if not passed:
                errors.append(f"lifecycle operation failed: {category} for {subject}")
            imported.append(
                Result(
                    category,
                    subject,
                    passed,
                    record["detail"],
                    record["returncode"],
                    "verified" if passed else "failed",
                )
            )

    extra_keys = set(records) - expected_keys
    if extra_keys:
        errors.append("sidecar contains lifecycle keys outside the supported lane matrix")
    passed_count = sum(result.passed and result.evidence_status == "verified" for result in imported)
    complete = passed_count == len(expected_keys) and not errors and len(records) == len(expected_keys)
    summary = {
        "record_count": record_count,
        "expected_record_count": len(expected_keys),
        "sha256": digest,
        "verified_frameworks": sorted(
            framework
            for framework in WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK
            if all(
                any(
                    result.category == category
                    and result.subject == f"web/{framework}"
                    and result.passed
                    and result.evidence_status == "verified"
                    for result in imported
                )
                for category in FRAMEWORK_RUNTIME_CATEGORIES
            )
        ),
        "errors": errors[:20],
    }
    imported.append(
        Result(
            "framework-lifecycle-evidence-source",
            "global",
            complete,
            json.dumps(summary, sort_keys=True),
            0 if complete else 1,
            "verified" if complete else "failed",
        )
    )
    return imported


def framework_lifecycle_evidence_summary(results: list[Result]) -> dict[str, Any]:
    """Describe sidecar coverage without extending it beyond its tested lanes.
    Mô tả phạm vi sidecar, không suy rộng ra ngoài lane đã chạy.
    """
    source = next(
        (result for result in results if result.category == "framework-lifecycle-evidence-source"),
        None,
    )
    if source is None:
        return {
            "status": "not-provided",
            "scope": "18 native web lanes, five CLI lifecycle operations per lane",
            "record_count": 0,
            "expected_record_count": len(WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK)
            * len(FRAMEWORK_RUNTIME_CATEGORIES),
            "sha256": None,
            "verified_frameworks": [],
        }
    try:
        detail = json.loads(source.detail)
    except json.JSONDecodeError:
        detail = {}
    return {
        "status": "complete" if source.passed and source.evidence_status == "verified" else "invalid-or-incomplete",
        "scope": "18 native web lanes, five CLI lifecycle operations per lane",
        "record_count": detail.get("record_count", 0),
        "expected_record_count": detail.get(
            "expected_record_count",
            len(WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK) * len(FRAMEWORK_RUNTIME_CATEGORIES),
        ),
        "sha256": detail.get("sha256"),
        "verified_frameworks": detail.get("verified_frameworks", []),
        "errors": detail.get("errors", []),
    }


def source_language_inventory_summary(results: list[Result]) -> dict[str, Any]:
    """Summarize generated source-language evidence, never runtime claims.
    Tóm tắt bằng chứng ngôn ngữ trong scaffold, không gán thành runtime claim.
    """
    frameworks: list[dict[str, Any]] = []
    by_language: dict[str, dict[str, int]] = defaultdict(
        lambda: {"frameworks": 0, "source_files": 0}
    )
    for result in results:
        if result.category != "framework-language-scaffold":
            continue
        try:
            evidence = json.loads(result.detail)
        except json.JSONDecodeError:
            evidence = {}
        languages = evidence.get("languages_or_source_formats", [])
        files_by_language = evidence.get("source_files_by_language_or_format", {})
        if not isinstance(languages, list):
            languages = []
        if not isinstance(files_by_language, dict):
            files_by_language = {}
        frameworks.append(
            {
                "subject": result.subject,
                "evidence_status": result.evidence_status,
                "languages_or_source_formats": languages,
                "source_files_by_language_or_format": files_by_language,
                "runtime_verified": False,
            }
        )
        for language in languages:
            if not isinstance(language, str):
                continue
            by_language[language]["frameworks"] += 1
            count = files_by_language.get(language, 0)
            if isinstance(count, int) and not isinstance(count, bool) and count >= 0:
                by_language[language]["source_files"] += count
    return {
        "evidence_scope": "generated scaffold files only; file extensions are not runtime proof",
        "runtime_verified": False,
        "frameworks_with_inventory": len(frameworks),
        "by_language_or_format": dict(sorted(by_language.items())),
        "frameworks": frameworks,
    }


def collect_source_provenance(binary: Path, repo_root: Path | None = None) -> dict[str, Any]:
    """Bind stress evidence to the source revision and exact CLI binary.
    Gắn stress evidence với revision source và chính xác binary CLI.
    """
    source_root = repo_root or Path(__file__).resolve().parents[2]
    digest = hashlib.sha256()
    try:
        with binary.open("rb") as executable:
            while chunk := executable.read(1024 * 1024):
                digest.update(chunk)
    except OSError:
        binary_sha256 = None
    else:
        binary_sha256 = digest.hexdigest()

    try:
        revision = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=source_root,
            text=True,
            encoding="utf-8",
            errors="replace",
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=SOURCE_PROVENANCE_TIMEOUT_SECONDS,
            check=False,
        )
        status = subprocess.run(
            ["git", "status", "--porcelain=v1"],
            cwd=source_root,
            text=True,
            encoding="utf-8",
            errors="replace",
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=SOURCE_PROVENANCE_TIMEOUT_SECONDS,
            check=False,
        )
    except (OSError, subprocess.TimeoutExpired):
        git_commit = None
        working_tree_clean = None
    else:
        commit_text = revision.stdout.strip()
        git_commit = (
            commit_text
            if revision.returncode == 0 and re.fullmatch(r"[0-9a-fA-F]{40,64}", commit_text)
            else None
        )
        working_tree_clean = (
            not status.stdout.strip()
            if status.returncode == 0
            else None
        )

    return {
        "git_commit": git_commit,
        "working_tree_clean": working_tree_clean,
        "binary_sha256": binary_sha256,
    }


def evidence_category_gaps(
    results: list[Result],
    rows: list[dict[str, Any]],
    categories: tuple[str, ...],
) -> list[str]:
    """Require exactly one verified runtime result for every lane/category.
    Bắt buộc mỗi lane/chiều có đúng một kết quả runtime đã xác minh.
    """
    by_key: dict[tuple[str, str], list[Result]] = defaultdict(list)
    for result in results:
        by_key[(result.category, result.subject)].append(result)

    gaps: list[str] = []
    for row in sorted(rows, key=lambda item: (item["core"], item["framework"])):
        subject = f"{row['core']}/{row['framework']}"
        for category in categories:
            matches = by_key.get((category, subject), [])
            if len(matches) != 1:
                gaps.append(f"{category}:{subject} expected exactly one runtime result, found {len(matches)}")
                continue
            result = matches[0]
            if not result.passed or result.evidence_status != "verified":
                gaps.append(
                    f"{category}:{subject} is {result.evidence_status} (passed={result.passed})"
                )
    return gaps


def framework_runtime_verified(results: list[Result], rows: list[dict[str, Any]]) -> bool:
    """Report runtime support only after every framework CLI operation is verified.
    Chỉ ghi runtime đã xác minh khi mọi framework có evidence CLI đủ chiều.
    """
    return bool(rows) and not evidence_category_gaps(
        results,
        rows,
        FRAMEWORK_RUNTIME_CATEGORIES,
    )


def framework_language_runtime_gaps(
    results: list[Result], rows: list[dict[str, Any]]
) -> list[str]:
    """Require runtime evidence for every standalone language in each scaffold.
    Bắt buộc có runtime evidence cho từng ngôn ngữ độc lập mà scaffold sinh ra.
    """
    by_key: dict[tuple[str, str], list[Result]] = defaultdict(list)
    for result in results:
        by_key[(result.category, result.subject)].append(result)

    gaps: list[str] = []
    for row in sorted(rows, key=lambda item: (item["core"], item["framework"])):
        subject = f"{row['core']}/{row['framework']}"
        scaffold_results = by_key.get(("framework-language-scaffold", subject), [])
        if len(scaffold_results) != 1:
            gaps.append(
                f"framework-language-scaffold:{subject} expected exactly one result, found {len(scaffold_results)}"
            )
            continue

        scaffold_result = scaffold_results[0]
        if not scaffold_result.passed or scaffold_result.evidence_status != "verified":
            gaps.append(
                f"framework-language-scaffold:{subject} is "
                f"{scaffold_result.evidence_status} (passed={scaffold_result.passed})"
            )
            continue
        try:
            evidence = json.loads(scaffold_result.detail)
        except json.JSONDecodeError:
            gaps.append(f"framework-language-scaffold:{subject} has malformed language inventory")
            continue

        languages = evidence.get("languages_or_source_formats") if isinstance(evidence, dict) else None
        if not isinstance(languages, list) or any(not isinstance(item, str) for item in languages):
            gaps.append(f"framework-language-scaffold:{subject} has malformed language inventory")
            continue

        for language in sorted(set(languages) - NON_STANDALONE_RUNTIME_FORMATS):
            probe_subject = f"{subject}/{language}"
            probes = by_key.get(("framework-language-runtime-probe", probe_subject), [])
            if len(probes) != 1:
                gaps.append(
                    f"framework-language-runtime-probe:{probe_subject} expected exactly one result, found {len(probes)}"
                )
                continue
            probe = probes[0]
            platform_only = language in {"Objective-C", "Objective-C++"} and sys.platform != "darwin"
            if platform_only and probe.passed and probe.evidence_status == "not_applicable":
                continue
            if not probe.passed or probe.evidence_status != "verified":
                gaps.append(
                    f"framework-language-runtime-probe:{probe_subject} is "
                    f"{probe.evidence_status} (passed={probe.passed})"
                )

    return gaps


def universal_support_gaps(results: list[Result], rows: list[dict[str, Any]]) -> list[str]:
    """Fail closed until every framework/core dimension has runtime evidence.
    Fail closed đến khi mọi framework/core có evidence runtime cho các chiều yêu cầu.
    """
    if not rows:
        return ["framework-inventory is empty"]

    gaps = evidence_category_gaps(results, rows, UNIVERSAL_FRAMEWORK_CATEGORIES)
    gaps.extend(framework_language_runtime_gaps(results, rows))
    for row in sorted(rows, key=lambda item: (item["core"], item["framework"])):
        subject = f"{row['core']}/{row['framework']}"
        ownership = row.get("dependency_ownership")
        if not isinstance(ownership, dict):
            gaps.append(f"dependency-owner:{subject} ownership declaration is missing")
            continue
        for operation in DEPENDENCY_OPERATIONS:
            cell = ownership.get(operation)
            owner = cell.get("owner") if isinstance(cell, dict) else None
            if owner != "mgc-native":
                gaps.append(f"dependency-owner:{subject}:{operation} is {owner or 'missing'}")

    for category in UNIVERSAL_GLOBAL_CATEGORIES:
        matches = [result for result in results if result.category == category]
        if not matches:
            gaps.append(f"{category}: global command contract has no evidence")
            continue
        for result in matches:
            if not result.passed or result.evidence_status != "verified":
                gaps.append(
                    f"{category}:{result.subject} is {result.evidence_status} (passed={result.passed})"
                )
    return gaps


def dependency_ownership_summary(rows: list[dict[str, Any]]) -> dict[str, Any]:
    """Summarize declared package-operation owners without treating them as runtime proof.
    Tóm lược owner do CLI khai báo, không biến metadata thành bằng chứng runtime.
    """
    counts: dict[str, dict[str, Counter[str]]] = defaultdict(
        lambda: {
            operation: Counter({owner: 0 for owner in DEPENDENCY_OWNERS})
            for operation in DEPENDENCY_OPERATIONS
        }
    )
    frameworks: list[dict[str, Any]] = []
    for row in sorted(rows, key=lambda item: (item["core"], item["framework"])):
        ownership = row.get("dependency_ownership")
        if not isinstance(ownership, dict) or set(ownership) != set(DEPENDENCY_OPERATIONS):
            raise ValueError(
                f"framework {row.get('core')}/{row.get('framework')} has incomplete dependency ownership"
            )
        operation_owners: dict[str, dict[str, Any]] = {}
        for operation in DEPENDENCY_OPERATIONS:
            cell = ownership[operation]
            owner = cell.get("owner") if isinstance(cell, dict) else None
            if not isinstance(owner, str) or owner not in DEPENDENCY_OWNERS:
                raise ValueError(
                    f"framework {row.get('core')}/{row.get('framework')} "
                    f"operation {operation} has invalid owner {owner!r}"
                )
            counts[row["core"]][operation][owner] += 1
            operation_owners[operation] = dict(cell)
        frameworks.append(
            {
                "core": row["core"],
                "framework": row["framework"],
                "qualification_status": row["status"],
                "operations": operation_owners,
            }
        )

    return {
        "source": "mgc capabilities --json",
        "evidence_scope": "declaration-only; does not verify runtime behavior",
        "runtime_verified": False,
        "operations": list(DEPENDENCY_OPERATIONS),
        "by_core_operation": {
            core: {
                operation: dict(sorted(operation_counts.items()))
                for operation, operation_counts in sorted(operation_counts.items())
            }
            for core, operation_counts in sorted(counts.items())
        },
        "frameworks": frameworks,
    }


def lockfile_evidence_gaps(results: list[Result], rows: list[dict[str, Any]]) -> list[str]:
    """Require one honest V3/V4 result per framework and verified claimed writers.
    Bắt buộc mỗi framework có một kết quả V3/V4 trung thực và writer đã claim phải xanh.
    """
    by_result: dict[tuple[str, str], list[Result]] = defaultdict(list)
    for result in results:
        if result.category in {"mgc-lock-v3-empty-graph", "mgc-lock-v4-empty-graph"}:
            by_result[(result.category, result.subject)].append(result)

    gaps: list[str] = []
    for row in sorted(rows, key=lambda item: (item["core"], item["framework"])):
        subject = f"{row['core']}/{row['framework']}"
        native_install = row["dependency_ownership"]["install"]["owner"] == "mgc-native"
        expected = {
            "mgc-lock-v3-empty-graph": {"verified"} if native_install else {"unsupported"},
            "mgc-lock-v4-empty-graph": {"verified"}
            if native_install and row["core"] in V4_INSTALL_CORES
            else {"unsupported"},
        }
        for category, expected_status in expected.items():
            matches = by_result.get((category, subject), [])
            if len(matches) != 1:
                gaps.append(f"{category}:{subject} expected one result, found {len(matches)}")
                continue
            result = matches[0]
            valid_evidence = result.evidence_status in expected_status and result.passed
            if not valid_evidence:
                gaps.append(
                    f"{category}:{subject} expected one of {sorted(expected_status)}, got "
                    f"{result.evidence_status} (passed={result.passed})"
                )
    return gaps


def full_evidence_gaps(results: list[Result]) -> list[Result]:
    """List unresolved optimizer/security evidence covered by the normal CI gate.
    Liệt kê evidence optimizer/security chưa giải quyết thuộc gate CI thông thường.
    """
    return [
        result
        for result in results
        if result.category in FULL_EVIDENCE_CATEGORIES
        and result.evidence_status == "unverified"
    ]


def language_runtime_evidence_gaps(results: list[Result]) -> list[Result]:
    """Return source-language probes without verified execution/parser evidence.
    Liệt kê probe ngôn ngữ nguồn chưa có bằng chứng chạy hoặc phân tích cú pháp.
    """
    return [
        result
        for result in results
        if result.category == "framework-language-runtime-probe"
        and not (
            result.passed and result.evidence_status == "not_applicable"
        )
        and (not result.passed or result.evidence_status != "verified")
    ]


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", default="target/debug/mgc", help="Path to the built mgc executable")
    parser.add_argument("--report", default="stress-test-report.json", help="Path for the JSON evidence report")
    parser.add_argument("--framework-timeout", type=int, default=90, help="Timeout per framework create command")
    parser.add_argument("--keep-workspace", help="Copy the isolated fixture tree here when any check fails")
    parser.add_argument(
        "--inventory-manifest",
        type=Path,
        default=DEFAULT_INVENTORY_BASELINE,
        help="Versioned expected framework/status/owner inventory",
    )
    parser.add_argument(
        "--require-full-evidence",
        action="store_true",
        help="Exit non-zero when any optimizer or security-audit result has only unverified evidence",
    )
    parser.add_argument(
        "--require-language-runtime-evidence",
        action="store_true",
        help="Exit non-zero when any standalone source-language probe is missing, unverified, or failed",
    )
    parser.add_argument(
        "--require-lockfile-evidence",
        action="store_true",
        help="Require empty-graph V3/V4 lock replay for every framework with a wired native install owner",
    )
    parser.add_argument(
        "--require-universal-support",
        action="store_true",
        help=(
            "Fail unless every framework has verified language runtime and CLI install/run/test/build/dev, optimizer, cache, "
            "security, shared-store, and V3/V4 evidence plus native ownership of all dependency operations"
        ),
    )
    args = parser.parse_args(argv)

    binary = resolve_binary(args.binary)
    results: list[Result] = []
    report_path = Path(args.report).expanduser().resolve()
    with tempfile.TemporaryDirectory(prefix="mgc-stress-matrix-") as temporary:
        root = Path(temporary)
        env = isolated_environment(root)
        try:
            version, rows = capability_rows(binary, root, env)
        except (RuntimeError, OSError, subprocess.TimeoutExpired) as error:
            print(f"FAIL capability manifest: {error}", file=sys.stderr)
            return 1
        try:
            inventory_count = check_framework_inventory(rows, args.inventory_manifest)
        except RuntimeError as error:
            print(f"FAIL framework inventory baseline: {error}", file=sys.stderr)
            return 1

        results.extend(run_cli_contracts(binary, root, env))
        scaffold_results, projects = create_frameworks(
            binary, rows, root, env, args.framework_timeout
        )
        results.extend(scaffold_results)
        results.extend(run_project_dimensions(binary, projects, rows, root, env))
        results.extend(
            read_web_lifecycle_evidence(
                os.environ.get("MGC_STRESS_LIFECYCLE_EVIDENCE"), binary
            )
        )
        support_gaps = universal_support_gaps(results, rows)
        write_report(
            report_path,
            version,
            results,
            rows,
            universal_gate_enforced=args.require_universal_support,
            support_gaps=support_gaps,
            source_provenance=collect_source_provenance(binary),
        )
        failed = [result for result in results if not result.passed]
        unverified = [result for result in results if result.evidence_status == "unverified"]
        full_evidence = full_evidence_gaps(results)
        language_runtime_gaps = language_runtime_evidence_gaps(results)
        unsupported = [result for result in results if result.evidence_status == "unsupported"]
        not_applicable = [result for result in results if result.evidence_status == "not_applicable"]
        lock_gaps = lockfile_evidence_gaps(results, rows)
        if failed and args.keep_workspace:
            keep_path = Path(args.keep_workspace).expanduser().resolve()
            if keep_path.exists():
                shutil.rmtree(keep_path)
            shutil.copytree(root, keep_path)

    print(f"MagiCore {version} on {platform.platform()}")
    print(
        f"Framework inventory baseline matched: {inventory_count} rows "
        f"({args.inventory_manifest})"
    )
    print(
        f"Frameworks: {len(rows)}; checks: {len(results)}; contract passes: {len(results) - len(failed)}; "
        f"failed: {len(failed)}; unverified evidence: {len(unverified)}"
    )
    print(
        f"Unsupported evidence: {len(unsupported)}; not applicable: {len(not_applicable)}; "
        "expected refusals are not support claims"
    )
    print(
        f"Verified evidence checks: {sum(r.passed and r.evidence_status == 'verified' for r in results)}; "
        "framework runtime is not verified by this runner"
    )
    print(f"Unverified optimizer/security evidence: {len(full_evidence)}")
    print(f"Unverified standalone language runtime/parser probes: {len(language_runtime_gaps)}")
    print(
        f"Lockfile evidence gaps: {len(lock_gaps)} "
        f"(empty-graph replay; V4 writers: {','.join(sorted(V4_INSTALL_CORES))})"
    )
    native_install = sum(
        row["dependency_ownership"]["install"]["owner"] == "mgc-native" for row in rows
    )
    print(
        f"Declared native install ownership: {native_install}/{len(rows)} frameworks; "
        "ownership is capability metadata, not runtime verification"
    )
    print(
        f"Universal support gate: {'ENFORCED' if args.require_universal_support else 'diagnostic'}; "
        f"{len(support_gaps)} evidence/ownership gap(s)"
    )
    for gap in support_gaps[:MAX_SUPPORT_GAP_DETAILS]:
        print(f"UNIVERSAL SUPPORT GAP: {gap}")
    if len(support_gaps) > MAX_SUPPORT_GAP_DETAILS:
        print(f"... and {len(support_gaps) - MAX_SUPPORT_GAP_DETAILS} more; see the JSON evidence report")
    print(f"Evidence: {report_path}")
    if failed and args.keep_workspace:
        print(f"Failed fixtures: {Path(args.keep_workspace).expanduser().resolve()}")
    for result in failed:
        print(f"FAIL [{result.category}] {result.subject}: {result.detail}")
    for gap in lock_gaps:
        print(f"LOCKFILE EVIDENCE GAP: {gap}")
    if unverified:
        print("UNVERIFIED evidence: " + ", ".join(f"{r.category}:{r.subject}" for r in unverified[:20]))
        if len(unverified) > 20:
            print(f"... and {len(unverified) - 20} more; see the JSON report")
    return 1 if (
        failed
        or (args.require_full_evidence and full_evidence)
        or (args.require_language_runtime_evidence and language_runtime_gaps)
        or (args.require_lockfile_evidence and lock_gaps)
        or (args.require_universal_support and support_gaps)
    ) else 0


if __name__ == "__main__":
    raise SystemExit(main())
