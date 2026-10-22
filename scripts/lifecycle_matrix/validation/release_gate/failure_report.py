"""Emit bounded fail-closed annotations for lifecycle gate failures."""

from __future__ import annotations

import json

from ...diagnostics.diagnostics import github_actions_error_annotations
from .models import GateState


def report_failures(state: GateState) -> None:
    for failure in state.failures:
        print("LIFECYCLE GATE FAILURE: " + json.dumps(failure))
    for annotation in github_actions_error_annotations(state.failures):
        print(annotation)
    print(f"LIFECYCLE NON-REGRESSION BLOCKED: {len(state.failures)} gate failures")

