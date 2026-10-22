"""Wires focused child owners — nối các owner con.
"""

from . import scaffold_language_files as _scaffold_language_files
from .scaffold_language_files import *

__all__ = [name for names in (_scaffold_language_files.__all__,) for name in names]
