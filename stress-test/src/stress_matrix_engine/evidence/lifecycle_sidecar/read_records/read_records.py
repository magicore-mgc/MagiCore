"""Owns bounded reading and validation of lifecycle sidecar records.
Sở hữu đọc có giới hạn và kiểm tra record lifecycle sidecar.
"""

from __future__ import annotations

import hashlib
import json
from collections import defaultdict
from pathlib import Path
from typing import Any

from ....constants.evidence_categories.evidence_categories import FRAMEWORK_RUNTIME_CATEGORIES, MAX_WEB_LIFECYCLE_EVIDENCE_BYTES
from ....constants.framework_catalog.framework_catalog import WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK
from ..types.types import LifecycleEvidence

def read_sidecar_records(sidecar: Path, binary: Path) -> LifecycleEvidence:
    """Read bounded JSONL and validate its identity, schema, and result fields.
    Đọc JSONL có giới hạn và xác minh identity, schema, trường kết quả.
    """
    evidence = LifecycleEvidence()
    binary_digest = hashlib.sha256()
    try:
        with binary.open("rb") as executable:
            while chunk := executable.read(1024 * 1024):
                binary_digest.update(chunk)
        expected_binary_sha256 = binary_digest.hexdigest()
    except OSError:
        expected_binary_sha256 = None
        evidence.errors.append("cannot hash the CLI binary")

    records: dict[tuple[str, str], list[dict[str, Any]]] = defaultdict(list)
    try:
        with sidecar.open("rb") as evidence_file:
            raw = evidence_file.read(MAX_WEB_LIFECYCLE_EVIDENCE_BYTES + 1)
            over_limit = len(raw) > MAX_WEB_LIFECYCLE_EVIDENCE_BYTES
            if not over_limit:
                over_limit = bool(evidence_file.read(1))
        if over_limit:
            evidence.errors.append("sidecar exceeds the 1 MiB evidence limit")
        else:
            evidence.digest = hashlib.sha256(raw).hexdigest()
            lines = raw.decode("utf-8").splitlines()
            if not lines:
                evidence.errors.append("sidecar is empty")
            for line_number, line in enumerate(lines, start=1):
                evidence.record_count += 1
                try:
                    value = json.loads(line)
                except json.JSONDecodeError:
                    evidence.errors.append(f"line {line_number}: malformed JSON")
                    continue
                if not isinstance(value, dict):
                    evidence.errors.append(f"line {line_number}: record must be a JSON object")
                    continue
                framework = value.get("framework")
                category = value.get("category")
                if (
                    not isinstance(framework, str)
                    or framework not in WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK
                ):
                    evidence.errors.append(f"line {line_number}: unknown framework")
                    continue
                if category not in FRAMEWORK_RUNTIME_CATEGORIES:
                    evidence.errors.append(f"line {line_number}: unknown lifecycle category")
                    continue
                expected_suite = WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK[framework]
                if value.get("schema_version") != 1 or isinstance(value.get("schema_version"), bool):
                    evidence.errors.append(f"line {line_number}: unsupported evidence schema")
                    continue
                if value.get("core") != "web" or value.get("suite") != expected_suite:
                    evidence.errors.append(f"line {line_number}: core or suite does not match the framework")
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
                    evidence.errors.append(f"line {line_number}: malformed result fields")
                    continue
                if passed != (returncode == 0):
                    evidence.errors.append(f"line {line_number}: pass status conflicts with exit code")
                    continue
                recorded_binary = value.get("binary_path")
                if not isinstance(recorded_binary, str) or not recorded_binary.strip():
                    evidence.errors.append(f"line {line_number}: binary path is missing")
                    continue
                try:
                    same_binary = Path(recorded_binary).expanduser().resolve() == binary.resolve()
                except (OSError, RuntimeError):
                    same_binary = False
                if not same_binary:
                    evidence.errors.append(f"line {line_number}: evidence belongs to a different CLI binary")
                    continue
                if value.get("binary_sha256") != expected_binary_sha256 or expected_binary_sha256 is None:
                    evidence.errors.append(f"line {line_number}: CLI binary digest is missing or mismatched")
                    continue
                records[(framework, category)].append(value)
    except (OSError, UnicodeError) as error:
        evidence.errors.append(f"cannot read sidecar: {error.__class__.__name__}")
    evidence.records = records
    return evidence

__all__ = ["read_sidecar_records"]
