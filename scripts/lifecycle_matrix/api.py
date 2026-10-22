"""Stable import API for lifecycle capability evidence."""

from __future__ import annotations

import datetime
import json
import os
import platform
import re
import secrets
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from typing import Optional

from .catalog.catalog import (
    LANES,
)
from .contract.dimensions import (
    ALL_DIMENSIONS,
    LANE_REQUIRED_DIMENSIONS,
    required_dimensions_for_lane,
)
from .contract.operations import (
    ALL_DEPENDENCY_OPERATIONS,
    NATIVE_INSTALL_PIPELINE_OPERATIONS,
    NATIVE_OPERATIONS_EXCEPT_GC,
    NATIVE_OPERATIONS_EXCEPT_GC_AND_OFFLINE,
    NATIVE_PM_REQUIRED_DIMENSIONS,
    NATIVE_PM_USER_OPERATIONS,
    NATIVE_USER_OPERATION_LANES,
    REQUIRED_OWNER_OPERATIONS,
)
from .contract.ownership import (
    DEPENDENCY_OWNER_VOCABULARY,
    LIFECYCLE_OWNER_VALUES,
    LIFECYCLE_STEP_OWNER_DEFAULTS,
    NATIVE_LIFECYCLE_OWNER_EVIDENCE,
    _owner_pass_status,
    lifecycle_owner_for,
    lifecycle_pass_status,
    lifecycle_status_for_owner,
    lifecycle_status_owner_matches,
)
from .contract.status import (
    ALL_STATUSES,
    LIFECYCLE_VERDICTS,
    PASS_STATUSES,
    SCHEMA_VERSION,
    STATUS_FAILED,
    STATUS_MANAGED,
    STATUS_NATIVE,
    STATUS_PLAIN,
    STATUS_UNSUPPORTED,
    STATUS_UNVERIFIED,
)
from .diagnostics.diagnostics import (
    COMMAND_OUTPUT_EXCERPT_LIMIT,
    command_output_excerpt,
    github_actions_annotation_escape,
    github_actions_error_annotations,
    lifecycle_failure_excerpt,
)
from .environment.flutter import (
    flutter_sdk_home_directory,
    flutter_sdk_pub_cache_directory,
)
from .environment.pulumi import (
    PULUMI_LIFECYCLE_STACK,
    pulumi_local_environment,
)
from .environment.python import (
    mgc_add_python_test_build_tools,
)
from .environment.recovery import (
    recovery_environment,
)
from .environment.shared import (
    lifecycle_environment,
    lifecycle_step_environment,
)
from .environment.swift import (
    swift_toolchain_environment,
)
from .errors import (
    _fail,
)
from .evidence.platform import (
    _platform_evidence_load,
    _platform_evidence_write,
    record_platform_green,
    reset_platform_counter,
)
from .evidence.records import (
    FRAMEWORK_QUALIFICATION_EVIDENCE,
    PLATFORM_EVIDENCE_OS_NAMES,
    PLATFORM_EVIDENCE_PATH,
    PLATFORM_EVIDENCE_PROMOTION_THRESHOLD,
    PLATFORM_EVIDENCE_RELEASE_SCOPE,
)
from .evidence.source import (
    lifecycle_platform_name,
    lifecycle_source_matches,
    lifecycle_working_tree_clean,
)
from .filesystem.filesystem import (
    _dir_nonempty,
)
from .filesystem.lockfiles import (
    _lockfile_candidates,
    _lockfile_entry_count,
)
from .filesystem.markers import (
    _materialize_marker,
)
from .paths.paths import (
    REPOSITORY_ROOT,
)
from .process.process import (
    run_text_capture,
)
from .runner.context import (
    STEP_TIMEOUT_DEFAULT_S,
)
from .runner.recovery import (
    RECOVERY_PHASE,
    RECOVERY_READY_TIMEOUT_S,
    _recovery_probe,
)
from .runner.runner import (
    run_lane,
)
from .validation.adapter import (
    ADAPTER_REGISTRY_CAPABILITIES,
    CLI_NATIVE_DEPENDENCY_ROUTES,
    validate_adapter_consistency,
)
from .validation.dep_gate import (
    check_dep_gate_consistency,
    validate_dep_gate_consistency,
)
from .validation.dependency_owners import (
    validate_dependency_owners,
)
from .validation.framework import (
    framework_catalog_errors,
    print_dependency_owner_summary,
)
from .validation.lane_owners import (
    validate_lane_owners,
)
from .validation.lane_registry import (
    validate_lane_registry,
)
from .validation.native_pm import (
    LANE_RELEASE_SCOPE_OUT_REASONS,
    evidence_commit_error,
    lifecycle_lane_owner_contract_errors,
    native_pm_claim_errors,
    native_pm_claim_scope,
    native_pm_lane_errors,
    native_pm_supported,
    native_pm_unavailable_verdict,
    release_scope_out_reason,
)
from .validation.platform import (
    platform_evidence_errors,
)
from .validation.scaffold import (
    SCAFFOLD_MARKERS,
    scaffold_language_matches,
)
from .cli.main import main

__all__ = ['ADAPTER_REGISTRY_CAPABILITIES', 'ALL_DEPENDENCY_OPERATIONS', 'ALL_DIMENSIONS', 'ALL_STATUSES', 'CLI_NATIVE_DEPENDENCY_ROUTES', 'COMMAND_OUTPUT_EXCERPT_LIMIT', 'DEPENDENCY_OWNER_VOCABULARY', 'FRAMEWORK_QUALIFICATION_EVIDENCE', 'LANES', 'LANE_RELEASE_SCOPE_OUT_REASONS', 'LANE_REQUIRED_DIMENSIONS', 'LIFECYCLE_OWNER_VALUES', 'LIFECYCLE_STEP_OWNER_DEFAULTS', 'LIFECYCLE_VERDICTS', 'NATIVE_INSTALL_PIPELINE_OPERATIONS', 'NATIVE_LIFECYCLE_OWNER_EVIDENCE', 'NATIVE_OPERATIONS_EXCEPT_GC', 'NATIVE_OPERATIONS_EXCEPT_GC_AND_OFFLINE', 'NATIVE_PM_REQUIRED_DIMENSIONS', 'NATIVE_PM_USER_OPERATIONS', 'NATIVE_USER_OPERATION_LANES', 'Optional', 'PASS_STATUSES', 'PLATFORM_EVIDENCE_OS_NAMES', 'PLATFORM_EVIDENCE_PATH', 'PLATFORM_EVIDENCE_PROMOTION_THRESHOLD', 'PLATFORM_EVIDENCE_RELEASE_SCOPE', 'PULUMI_LIFECYCLE_STACK', 'Path', 'RECOVERY_PHASE', 'RECOVERY_READY_TIMEOUT_S', 'REPOSITORY_ROOT', 'REQUIRED_OWNER_OPERATIONS', 'SCAFFOLD_MARKERS', 'SCHEMA_VERSION', 'STATUS_FAILED', 'STATUS_MANAGED', 'STATUS_NATIVE', 'STATUS_PLAIN', 'STATUS_UNSUPPORTED', 'STATUS_UNVERIFIED', 'STEP_TIMEOUT_DEFAULT_S', '_dir_nonempty', '_fail', '_lockfile_candidates', '_lockfile_entry_count', '_materialize_marker', '_owner_pass_status', '_platform_evidence_load', '_platform_evidence_write', '_recovery_probe', 'check_dep_gate_consistency', 'command_output_excerpt', 'datetime', 'evidence_commit_error', 'flutter_sdk_home_directory', 'flutter_sdk_pub_cache_directory', 'framework_catalog_errors', 'github_actions_annotation_escape', 'github_actions_error_annotations', 'json', 'lifecycle_environment', 'lifecycle_failure_excerpt', 'lifecycle_lane_owner_contract_errors', 'lifecycle_owner_for', 'lifecycle_pass_status', 'lifecycle_platform_name', 'lifecycle_source_matches', 'lifecycle_status_for_owner', 'lifecycle_status_owner_matches', 'lifecycle_step_environment', 'lifecycle_working_tree_clean', 'main', 'mgc_add_python_test_build_tools', 'native_pm_claim_errors', 'native_pm_claim_scope', 'native_pm_lane_errors', 'native_pm_supported', 'native_pm_unavailable_verdict', 'os', 'platform', 'platform_evidence_errors', 'print_dependency_owner_summary', 'pulumi_local_environment', 're', 'record_platform_green', 'recovery_environment', 'release_scope_out_reason', 'required_dimensions_for_lane', 'reset_platform_counter', 'run_lane', 'run_text_capture', 'scaffold_language_matches', 'secrets', 'shutil', 'subprocess', 'swift_toolchain_environment', 'sys', 'tempfile', 'time', 'validate_adapter_consistency', 'validate_dep_gate_consistency', 'validate_dependency_owners', 'validate_lane_owners', 'validate_lane_registry']
