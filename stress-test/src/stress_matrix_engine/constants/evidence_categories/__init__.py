"""Wires focused child owners — nối các owner con.
"""

from . import evidence_categories as _evidence_categories
from .evidence_categories import *

__all__ = [name for names in (_evidence_categories.__all__,) for name in names]
