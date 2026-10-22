"""CLI entry for the global lifecycle and native-PM release gate."""

from __future__ import annotations

import json
import sys
from pathlib import Path


def main() -> None:
    scripts_root = Path(__file__).resolve().parents[3]
    sys.path.insert(0, str(scripts_root))
    from lifecycle_matrix.validation.release_gate.context import prepare_gate
    from lifecycle_matrix.validation.release_gate.failure_report import report_failures
    from lifecycle_matrix.validation.release_gate.matrix import validate_lanes
    from lifecycle_matrix.validation.release_gate.scope import validate_scope
    from lifecycle_matrix.validation.release_gate.success_report import report_success

    with open("docs/specs/lifecycleCapabilityMatrix.json") as matrix_file:
        data = json.load(matrix_file)
    state = prepare_gate(data)
    validate_lanes(state)
    validate_scope(state)
    if state.failures:
        report_failures(state)
        raise SystemExit(1)
    report_success(state)


if __name__ == "__main__":
    main()

