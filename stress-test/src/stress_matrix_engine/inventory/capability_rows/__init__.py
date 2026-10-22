"""Wires focused child owners — nối các owner con.
"""

from . import capability_rows as _capability_rows
from .capability_rows import *

__all__ = [name for names in (_capability_rows.__all__,) for name in names]
