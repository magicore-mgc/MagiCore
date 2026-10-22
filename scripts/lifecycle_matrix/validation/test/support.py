"""Shared helpers for lifecycle matrix validation tests.

Các helper dùng chung cho test kiểm định lifecycle matrix.
"""

import io
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import unittest
from contextlib import redirect_stdout
from pathlib import Path, PureWindowsPath
from unittest.mock import patch

import lifecycle_capability_matrix as matrix_module
from lifecycle_capability_matrix import (
    NATIVE_PM_REQUIRED_DIMENSIONS,
    NATIVE_PM_USER_OPERATIONS,
    ALL_DEPENDENCY_OPERATIONS,
    PLATFORM_EVIDENCE_RELEASE_SCOPE,
    ALL_DIMENSIONS,
    LANES,
    validate_lane_registry,
    SCHEMA_VERSION,
    STATUS_NATIVE,
    STATUS_PLAIN,
    STATUS_FAILED,
    STATUS_UNVERIFIED,
    STATUS_UNSUPPORTED,
    native_pm_supported,
    native_pm_unavailable_verdict,
    native_pm_claim_scope,
    native_pm_claim_errors,
    native_pm_lane_errors,
    lifecycle_lane_owner_contract_errors,
    lifecycle_status_owner_matches,
    lifecycle_owner_for,
    lifecycle_pass_status,
    lifecycle_status_for_owner,
    framework_catalog_errors,
    release_scope_out_reason,
    evidence_commit_error,
    scaffold_language_matches,
    platform_evidence_errors,
    record_platform_green,
    print_dependency_owner_summary,
    lifecycle_platform_name,
    lifecycle_environment,
    pulumi_local_environment,
    recovery_environment,
    mgc_add_python_test_build_tools,
    validate_adapter_consistency,
    _materialize_marker,
    _lockfile_candidates,
    lifecycle_working_tree_clean,
    lifecycle_source_matches,
    run_lane,
)
from provenance_chain import _matrix_sha_for_head

SCRIPTS_ROOT = Path(__file__).resolve().parents[3]
REPO_ROOT = SCRIPTS_ROOT.parent

def bash_compatible_path(path, windows=None):
    value = os.fspath(path)
    if windows is None:
        windows = os.name == "nt"
    if not windows:
        return value

    windows_path = PureWindowsPath(value)
    drive = windows_path.drive.rstrip(":").lower()
    if not drive:
        raise ValueError(f"Windows path must include a drive letter: {value}")

    # Git Bash exposes local Windows drives below /<drive>/.
    # Git Bash hiển thị các ổ đĩa Windows cục bộ dưới /<drive>/.
    return f"/{drive}/" + "/".join(windows_path.parts[1:])

def bootstrap_bash_command(platform=None, git_executable=None):
    if platform is None:
        platform = os.name
    if platform != "nt":
        return "bash"
    if git_executable is None:
        git_executable = shutil.which("git")
    if not git_executable:
        return None

    # Bind platform choice and Windows shell resolution in one auditable function.
    # Gộp chọn shell theo hệ điều hành và tìm Bash trên Windows để audit chung.
    git_path = Path(git_executable).resolve()
    for install_directory in (git_path.parent, *git_path.parents):
        for relative_path in (Path("bin/bash.exe"), Path("usr/bin/bash.exe")):
            candidate = install_directory / relative_path
            if candidate.is_file():
                return str(candidate.resolve())
    return None

def evidence(dimensions=None, owners=None):
    statuses = {name: STATUS_NATIVE for name in NATIVE_PM_REQUIRED_DIMENSIONS}
    statuses.update(dimensions or {})
    operation_owners = {
        "resolve": "mgc",
        "lock": "mgc",
        "fetch": "mgc",
        "verify": "mgc",
        "store": "magicore-shared-cas",
        "materialize": "mgc",
    }
    operation_owners.update({operation: "mgc" for operation in NATIVE_PM_USER_OPERATIONS})
    operation_owners.update(owners or {})
    return statuses, operation_owners

class PlatformGreenMatrixFixture:
    """Build shared synthetic lifecycle matrices.

    Tạo fixture matrix lifecycle tổng hợp dùng chung.
    """

    def green_matrix(self):
        def catalog_owners(lane):
            return {
                operation: {
                    "owner": (
                        "mgc-native"
                        if matrix_owner in {"mgc", "magicore-shared-cas"}
                        else "scaffold-only"
                        if matrix_owner == "scaffold-only"
                        else "unsupported"
                    )
                }
                for operation, matrix_owner in lane["owner_by_operation"].items()
            }

        def native_pm_verdict(lane):
            dimensions = {
                **{name: STATUS_NATIVE for name in ALL_DIMENSIONS},
                **{
                    name: lifecycle_status_for_owner(
                        lifecycle_owner_for(lane, name)
                    )
                    for name in lane["required_dims"]
                },
            }
            if lane["dependency_owner"] == "mgc-native":
                return (
                    "native-pm-supported"
                    if native_pm_supported(
                        lane["dependency_owner"],
                        lane["install_owner"],
                        [],
                        dimensions,
                        lane["owner_by_operation"],
                    )
                    else "not-native-pm"
                )
            if lane["dependency_owner"] == "delegated":
                return "compatibility-passed"
            return "unsupported"

        return {
            "schema_version": SCHEMA_VERSION,
            "matrix_kind": "lifecycle",
            "commit": "a" * 40,
            "platform": "Windows",
            "working_tree_clean": True,
            "lanes": [
                {
                    "core": lane["core"],
                    "language": lane["language"],
                    "framework_id": lane.get("framework_id", ""),
                    "verdict": "orchestration-lifecycle-passed",
                    "evidence_only": bool(lane.get("evidence_only")),
                    "toolchain_available": True,
                    "required_dimensions": lane["required_dims"],
                    "dimensions": {
                        **{name: STATUS_NATIVE for name in ALL_DIMENSIONS},
                        **{
                            name: lifecycle_status_for_owner(
                                lifecycle_owner_for(lane, name)
                            )
                            for name in lane["required_dims"]
                        },
                    },
                    "lifecycle_owners": {
                        name: lifecycle_owner_for(lane, name)
                        for name in lane["required_dims"]
                    },
                    "dependency_owner": lane["dependency_owner"],
                    "install_owner": lane["install_owner"],
                    "native_pm_delegated": [],
                    "native_pm_verdict": native_pm_verdict(lane),
                    "owner_by_operation": dict(lane["owner_by_operation"]),
                    "release_scope_out_reason": release_scope_out_reason(
                        lane["core"], lane["language"], lane.get("framework_id", "")
                    ),
                    "gate_role": (
                        "scoped-out"
                        if release_scope_out_reason(
                            lane["core"], lane["language"], lane.get("framework_id", "")
                        )
                        else "release-blocking"
                    ),
                }
                for lane in LANES
            ],
            "framework_catalog": [
                {
                    "core": lane["core"],
                    "framework": lane["framework_id"],
                    "status": (
                        "mgc-engine-path"
                        if lane["dependency_owner"] == "mgc-native"
                        else "scaffold-only"
                    ),
                    "dependency_ownership": catalog_owners(lane),
                    "evidence": "synthetic passing test fixture",
                }
                for lane in LANES
                if lane.get("framework_id")
            ],
        }

__all__ = [
    'ALL_DEPENDENCY_OPERATIONS',
    'ALL_DIMENSIONS',
    'LANES',
    'NATIVE_PM_REQUIRED_DIMENSIONS',
    'NATIVE_PM_USER_OPERATIONS',
    'PLATFORM_EVIDENCE_RELEASE_SCOPE',
    'REPO_ROOT',
    'SCHEMA_VERSION',
    'SCRIPTS_ROOT',
    'STATUS_FAILED',
    'STATUS_NATIVE',
    'STATUS_PLAIN',
    'STATUS_UNSUPPORTED',
    'STATUS_UNVERIFIED',
    'Path',
    'PlatformGreenMatrixFixture',
    'PureWindowsPath',
    '_lockfile_candidates',
    '_materialize_marker',
    '_matrix_sha_for_head',
    'bash_compatible_path',
    'evidence',
    'framework_catalog_errors',
    'io',
    'json',
    'lifecycle_environment',
    'lifecycle_lane_owner_contract_errors',
    'lifecycle_owner_for',
    'lifecycle_pass_status',
    'lifecycle_platform_name',
    'lifecycle_source_matches',
    'lifecycle_status_for_owner',
    'lifecycle_status_owner_matches',
    'lifecycle_working_tree_clean',
    'matrix_module',
    'mgc_add_python_test_build_tools',
    'native_pm_claim_errors',
    'native_pm_claim_scope',
    'native_pm_lane_errors',
    'native_pm_supported',
    'native_pm_unavailable_verdict',
    'os',
    'patch',
    'platform_evidence_errors',
    'print_dependency_owner_summary',
    'pulumi_local_environment',
    're',
    'recovery_environment',
    'record_platform_green',
    'release_scope_out_reason',
    'run_lane',
    'scaffold_language_matches',
    'shutil',
    'subprocess',
    'sys',
    'tempfile',
    'unittest',
    'validate_adapter_consistency',
    'validate_lane_registry',
    'redirect_stdout',
    'bootstrap_bash_command',
    'evidence_commit_error',
]
