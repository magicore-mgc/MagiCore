"""Wires focused child owners — nối các owner con.
"""

from . import report as _report
from .report import *

__all__ = [name for names in (_report.__all__,) for name in names]
