"""Owns stress-matrix CLI orchestration.
Sở hữu điều phối CLI stress-matrix.
"""

from __future__ import annotations

from pathlib import Path

from ...process.resolve_binary.resolve_binary import resolve_binary

from .arguments.arguments import build_argument_parser
from .data_collection.data_collection import collect_matrix_data
from .summary.summary import render_matrix_summary

def main(argv: list[str] | None = None) -> int:
    """Parse the unchanged CLI, run the matrix, and render its report.
    Parse CLI không đổi, chạy matrix và in báo cáo.
    """
    args = build_argument_parser().parse_args(argv)
    binary = resolve_binary(args.binary)
    report_path = Path(args.report).expanduser().resolve()
    run = collect_matrix_data(args, binary, report_path)
    if run is None:
        return 1
    return render_matrix_summary(args, run)

__all__ = ["main"]
