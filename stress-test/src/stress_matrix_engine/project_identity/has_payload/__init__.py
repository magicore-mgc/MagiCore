"""Wires focused child owners — nối các owner con.
"""

from . import has_payload as _has_payload
from .has_payload import *

__all__ = [name for names in (_has_payload.__all__,) for name in names]
