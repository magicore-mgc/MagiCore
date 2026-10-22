"""Wires focused child owners — nối các owner con.
"""

from . import read_project_identity as _read_project_identity
from .read_project_identity import *

__all__ = [name for names in (_read_project_identity.__all__,) for name in names]
