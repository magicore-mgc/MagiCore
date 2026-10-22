"""Enforce exact release-lane coverage and native-PM claim completeness."""

from __future__ import annotations

from ...validation.native_pm import native_pm_claim_errors
from .models import GateState


def validate_scope(state: GateState) -> None:
    missing = state.release_scope - state.seen
    if missing:
        state.failures.append(
            "release-scope lanes missing from matrix: "
            + ", ".join(
                f"{core}/{language}" + (f"#{framework}" if framework else "")
                for core, language, framework in sorted(missing)
            )
        )
    state.failures.extend(native_pm_claim_errors(state.data["lanes"]))
    if state.seen != state.release_scope:
        state.failures.append(
            "global v1.2 lane set mismatch: "
            f"missing={sorted(state.release_scope - state.seen)}, "
            f"unexpected={sorted(state.seen - state.release_scope)}"
        )

