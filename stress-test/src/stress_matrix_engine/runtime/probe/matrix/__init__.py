"""Wires focused child owners — nối các owner con.
"""

from . import matrix as _matrix
from .matrix import *

__all__ = [name for names in (_matrix.__all__,) for name in names]
