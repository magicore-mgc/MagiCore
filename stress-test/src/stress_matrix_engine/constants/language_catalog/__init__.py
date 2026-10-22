"""Wires focused child owners — nối các owner con.
"""

from . import language_catalog as _language_catalog
from .language_catalog import *

__all__ = [name for names in (_language_catalog.__all__,) for name in names]
