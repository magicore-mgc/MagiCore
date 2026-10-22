"""Wires focused child owners — nối các owner con.
"""

from . import upsert_script_entries as _upsert_script_entries
from .upsert_script_entries import *

__all__ = [name for names in (_upsert_script_entries.__all__,) for name in names]
