"""Wires focused child owners — nối các owner con.
"""

from . import set_lock_schema as _set_lock_schema
from .set_lock_schema import *

__all__ = [name for names in (_set_lock_schema.__all__,) for name in names]
