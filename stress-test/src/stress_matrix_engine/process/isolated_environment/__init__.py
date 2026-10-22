"""Wires focused child owners — nối các owner con.
"""

from . import isolated_environment as _isolated_environment
from .isolated_environment import *

__all__ = [name for names in (_isolated_environment.__all__,) for name in names]
