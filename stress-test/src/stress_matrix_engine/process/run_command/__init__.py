"""Wires focused child owners — nối các owner con.
"""

from . import run_command as _run_command
from .run_command import *

__all__ = [name for names in (_run_command.__all__,) for name in names]
