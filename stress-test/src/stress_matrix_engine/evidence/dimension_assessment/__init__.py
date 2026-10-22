"""Wires focused child owners — nối các owner con.
"""

from . import dimension_assessment as _dimension_assessment
from .dimension_assessment import *

__all__ = [name for names in (_dimension_assessment.__all__,) for name in names]
