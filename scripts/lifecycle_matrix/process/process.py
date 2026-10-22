"""Stable text-capturing subprocess adapter."""

from __future__ import annotations

import subprocess

def run_text_capture(*args, **kwargs):
    """Run a child process with stable UTF-8 decoding for captured text.
    Chạy tiến trình con với giải mã UTF-8 ổn định cho output dạng text.
    """
    kwargs.setdefault("text", True)
    kwargs.setdefault("encoding", "utf-8")
    kwargs.setdefault("errors", "replace")
    return subprocess.run(*args, **kwargs)

__all__ = ['run_text_capture']
