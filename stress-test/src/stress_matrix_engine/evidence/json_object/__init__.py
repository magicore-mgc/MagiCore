"""Wires focused child owners — nối các owner con.
"""

from . import json_object as _json_object
from .json_object import *

__all__ = [name for names in (_json_object.__all__,) for name in names]
