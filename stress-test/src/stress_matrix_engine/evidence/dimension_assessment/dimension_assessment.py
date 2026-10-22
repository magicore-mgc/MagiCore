"""Owns the dimension assessment responsibility in the stress-matrix runner.
Sở hữu trách nhiệm dimension assessment trong stress-matrix runner.
"""

from __future__ import annotations

import subprocess

from ..json_object.json_object import _parse_json_object

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

__all__ = ['assess_dimension']
