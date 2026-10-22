"""Wires focused child owners — nối các owner con.
"""

from . import dependency_ownership as _dependency_ownership
from .dependency_ownership import *
from . import lifecycle as _lifecycle
from .lifecycle import *
from . import source_language as _source_language
from .source_language import *

__all__ = [name for names in (_dependency_ownership.__all__, _lifecycle.__all__, _source_language.__all__,) for name in names]
