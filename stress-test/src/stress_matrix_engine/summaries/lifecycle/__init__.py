"""Wires focused child owners — nối các owner con.
"""

from . import lifecycle as _lifecycle
from .lifecycle import *

__all__ = [name for names in (_lifecycle.__all__,) for name in names]
