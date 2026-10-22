"""Render lane verdicts and non-passing dimensions for CI logs."""

from __future__ import annotations

from ..contract.status import PASS_STATUSES

def render_lane_results(results: list[dict]) -> None:
    """Print the per-lane pass and non-pass evidence summary."""
    for r in results:
        passed = [d for d, s in r["dimensions"].items() if s in PASS_STATUSES]
        rest = {
            d: s for d, s in r["dimensions"].items() if s not in PASS_STATUSES
        }
        print(f"  {r['verdict']:<30} {r['core']}/{r['language']}: pass={passed}")
        print(f"      native_pm={r.get('native_pm_verdict')}")
        print(f"      non-pass={rest}")

__all__ = ["render_lane_results"]
