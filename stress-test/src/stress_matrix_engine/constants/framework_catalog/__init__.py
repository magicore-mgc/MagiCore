"""Wires focused child owners — nối các owner con.
"""

from . import framework_catalog as _framework_catalog
from .framework_catalog import *

__all__ = [name for names in (_framework_catalog.__all__,) for name in names]
