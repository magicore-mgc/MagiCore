"""Wires focused child owners — nối các owner con.
"""

from . import check_framework_inventory as _check_framework_inventory
from .check_framework_inventory import *

__all__ = [name for names in (_check_framework_inventory.__all__,) for name in names]
