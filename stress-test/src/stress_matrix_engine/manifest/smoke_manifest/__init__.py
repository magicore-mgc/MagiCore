"""Wires focused child owners — nối các owner con.
"""

from . import smoke_manifest as _smoke_manifest
from .smoke_manifest import *

__all__ = [name for names in (_smoke_manifest.__all__,) for name in names]
