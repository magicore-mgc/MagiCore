"""Wires focused child owners — nối các owner con.
"""

from . import parse_toml_string as _parse_toml_string
from .parse_toml_string import *

__all__ = [name for names in (_parse_toml_string.__all__,) for name in names]
