"""Wires focused child owners — nối các owner con.
"""

from . import language_runtime_profile as _language_runtime_profile
from .language_runtime_profile import *

__all__ = [name for names in (_language_runtime_profile.__all__,) for name in names]
