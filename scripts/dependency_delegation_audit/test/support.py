"""Shared imports and fixture helpers for dependency delegation audit tests."""

from __future__ import annotations

import ast
import json
import os
import sys
import tempfile
import unittest
from unittest import mock

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
from ..paths.paths import REPO_ROOT
import audit_dependency_delegation as gate

def scan_snippet(rel_path, snippet):
    """Run scan_file over an in-memory snippet via a temp file.
    (Chạy scan_file trên đoạn mã trong bộ nhớ qua file tạm.)"""
    with tempfile.NamedTemporaryFile(
        mode="w", suffix=".rs", delete=False, encoding="utf-8"
    ) as handle:
        handle.write(snippet)
        tmp = handle.name
    try:
        return gate.scan_file(rel_path, tmp)
    finally:
        os.unlink(tmp)

__all__ = ["ast", "json", "os", "sys", "tempfile", "unittest", "mock", "gate", "scan_snippet", "REPO_ROOT"]
