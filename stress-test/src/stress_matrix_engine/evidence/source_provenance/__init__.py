"""Wires focused child owners — nối các owner con.
"""

from . import source_provenance as _source_provenance
from .source_provenance import *

__all__ = [name for names in (_source_provenance.__all__,) for name in names]
