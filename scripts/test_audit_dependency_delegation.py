#!/usr/bin/env python3
"""Stable unittest entrypoint for dependency delegation audit checks."""

from __future__ import annotations

import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from dependency_delegation_audit.test import test as test_suite

if __name__ == "__main__":
    unittest.main(module=test_suite, verbosity=2)
