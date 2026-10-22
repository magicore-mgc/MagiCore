"""Owns store doctor runtime health classification.
Sở hữu phân loại health runtime của store doctor.
"""

from __future__ import annotations

from typing import Any

def evaluate_store_runtime_health(
    payload: dict[str, Any] | None, reported_scope: Any, scope_verified: bool, stdout: str
) -> tuple[bool, str, str]:
    """Classify store health evidence without upgrading missing proof.
    Phân loại store health mà không nâng cấp bằng chứng còn thiếu.
    """
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
        runtime_detail = stdout[-1200:]
    return runtime_check_passed, runtime_detail, runtime_evidence_status

__all__ = ["evaluate_store_runtime_health"]
