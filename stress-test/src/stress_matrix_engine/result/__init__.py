"""Wires focused child owners — nối các owner con.
"""

from . import result as _result
from .result import *

__all__ = [name for names in (_result.__all__,) for name in names]
