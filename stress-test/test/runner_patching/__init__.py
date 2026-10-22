"""Expose the refactor-aware runner test fixture.
Export fixture test runner theo owner sau refactor.
"""

from .runner_patching import patch_runner_symbol

__all__ = ["patch_runner_symbol"]
