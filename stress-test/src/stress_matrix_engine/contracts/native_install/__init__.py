"""Wires focused child owners — nối các owner con.
"""

from . import native_install as _native_install
from .native_install import *

__all__ = [name for names in (_native_install.__all__,) for name in names]
