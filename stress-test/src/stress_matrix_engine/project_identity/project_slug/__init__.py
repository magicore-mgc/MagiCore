"""Wires focused child owners — nối các owner con.
"""

from . import project_slug as _project_slug
from .project_slug import *

__all__ = [name for names in (_project_slug.__all__,) for name in names]
