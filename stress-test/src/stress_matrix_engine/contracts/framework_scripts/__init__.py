"""Wires focused child owners — nối các owner con.
"""

from . import framework_scripts as _framework_scripts
from .framework_scripts import *

__all__ = [name for names in (_framework_scripts.__all__,) for name in names]
