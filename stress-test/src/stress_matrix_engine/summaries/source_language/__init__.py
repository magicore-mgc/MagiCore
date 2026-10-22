"""Wires focused child owners — nối các owner con.
"""

from . import source_language as _source_language
from .source_language import *

__all__ = [name for names in (_source_language.__all__,) for name in names]
