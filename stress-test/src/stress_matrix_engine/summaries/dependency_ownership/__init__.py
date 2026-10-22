"""Wires focused child owners — nối các owner con.
"""

from . import dependency_ownership as _dependency_ownership
from .dependency_ownership import *

__all__ = [name for names in (_dependency_ownership.__all__,) for name in names]
