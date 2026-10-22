"""Wires focused child owners — nối các owner con.
"""

from . import project_dimensions as _project_dimensions
from .project_dimensions import *

__all__ = [name for names in (_project_dimensions.__all__,) for name in names]
