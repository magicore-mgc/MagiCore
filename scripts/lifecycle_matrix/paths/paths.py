"""Checkout paths used by the lifecycle evidence tool."""

from __future__ import annotations

from pathlib import Path

PACKAGE_ROOT = Path(__file__).resolve().parents[1]
SCRIPTS_ROOT = Path(__file__).resolve().parents[2]
REPOSITORY_ROOT = Path(__file__).resolve().parents[3]

__all__ = ["PACKAGE_ROOT", "SCRIPTS_ROOT", "REPOSITORY_ROOT"]
