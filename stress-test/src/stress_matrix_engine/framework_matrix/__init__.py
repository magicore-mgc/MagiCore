"""Wires focused child owners — nối các owner con.
"""

from . import create_frameworks as _create_frameworks
from .create_frameworks import *
from . import scaffold_language_files as _scaffold_language_files
from .scaffold_language_files import *

__all__ = [name for names in (_create_frameworks.__all__, _scaffold_language_files.__all__,) for name in names]
