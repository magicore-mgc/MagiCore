"""Lifecycle matrix CLI composition entrypoint."""

from __future__ import annotations

from .collect import collect_lane_results
from .document import build_matrix_document
from .output import write_matrix_document
from .preflight import prepare_matrix_run
from .report import render_lane_results
from .verdicts import apply_lane_verdicts

def main() -> int:
    context = prepare_matrix_run()
    if isinstance(context, int):
        return context
    results = collect_lane_results(context)
    apply_lane_verdicts(results)
    document = build_matrix_document(context, results)
    write_matrix_document(document, results)
    render_lane_results(results)
    return 0

__all__ = ["main"]
