"""Wires focused child owners — nối các owner con.
"""

from . import parse_toml_string_array as _parse_toml_string_array
from .parse_toml_string_array import *

__all__ = [name for names in (_parse_toml_string_array.__all__,) for name in names]
