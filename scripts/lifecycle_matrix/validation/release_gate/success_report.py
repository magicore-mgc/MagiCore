"""Print release-blocking, scoped-out, and qualified lane summaries."""

from __future__ import annotations

from .models import GateState


def _lane_name(lane: dict) -> str:
    return (
        f"{lane['core']}/{lane['language']}"
        + (f"#{lane.get('framework_id')}" if lane.get("framework_id") else "")
    )


def report_success(state: GateState) -> None:
    print(
        "OK: release-blocking lifecycle lanes passed: "
        + ", ".join(
            f"{core}/{language}" + (f"#{framework}" if framework else "")
            for core, language, framework in sorted(state.release_blocking_seen)
        )
    )
    lane_scope_outs = [
        f"{lane['core']}/{lane['language']}: {lane['release_scope_out_reason']}"
        for lane in state.data["lanes"]
        if lane.get("release_scope_out_reason")
    ]
    framework_scope_outs = [
        f"{row['core']}/{row['framework']}: {row['evidence']}"
        for row in state.data.get("framework_catalog", [])
        if row.get("status") == "scaffold-only"
    ]
    if lane_scope_outs:
        print(
            "Lifecycle-scoped-out lanes (native-PM claims remain gated): "
            + " | ".join(lane_scope_outs)
        )
    if framework_scope_outs:
        print("Explicitly scoped-out frameworks: " + " | ".join(framework_scope_outs))
    qualified = [
        lane for lane in state.data["lanes"]
        if lane.get("dependency_owner") == "mgc-native"
        and lane.get("native_pm_verdict") == "native-pm-supported"
    ]
    unqualified = [
        lane for lane in state.data["lanes"]
        if lane.get("dependency_owner") == "mgc-native"
        and lane.get("native_pm_verdict") != "native-pm-supported"
    ]
    print(
        "Fully native-PM qualified lanes: "
        + (", ".join(_lane_name(lane) for lane in qualified) or "none")
    )
    if unqualified:
        print(
            "Native-PM claim withheld pending complete runtime evidence: "
            + ", ".join(_lane_name(lane) for lane in unqualified)
        )
