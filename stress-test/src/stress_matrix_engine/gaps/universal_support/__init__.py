"""Wires focused child owners — nối các owner con.
"""

from . import universal_support as _universal_support
from .universal_support import *

__all__ = [name for names in (_universal_support.__all__,) for name in names]
