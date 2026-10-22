"""Wires focused child owners — nối các owner con.
"""

from . import dependency_operations as _dependency_operations
from .dependency_operations import *

__all__ = [name for names in (_dependency_operations.__all__,) for name in names]
