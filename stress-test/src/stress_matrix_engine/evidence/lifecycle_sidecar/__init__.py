"""Wires focused child owners — nối các owner con.
"""

from . import lifecycle_sidecar as _lifecycle_sidecar
from .lifecycle_sidecar import *

__all__ = [name for names in (_lifecycle_sidecar.__all__,) for name in names]
