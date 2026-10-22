"""Wires focused child owners — nối các owner con.
"""

from . import validate_scaffold as _validate_scaffold
from .validate_scaffold import *

__all__ = [name for names in (_validate_scaffold.__all__,) for name in names]
