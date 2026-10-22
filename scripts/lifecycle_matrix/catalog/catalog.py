"""Ordered aggregate of lifecycle profiles for every declared core."""

from __future__ import annotations

from .operation_owners import apply_operation_owners

from .lib import LANES as LIB_LANES
from .web import LANES as WEB_LANES
from .ai import LANES as AI_LANES
from .app import LANES as APP_LANES
from .game import LANES as GAME_LANES
from .iot import LANES as IOT_LANES
from .clo import LANES as CLO_LANES
from .hardware import LANES as HARDWARE_LANES
from .cicd import LANES as CICD_LANES

LANES = [
    *LIB_LANES,
    *WEB_LANES,
    *AI_LANES,
    *APP_LANES,
    *GAME_LANES,
    *IOT_LANES,
    *CLO_LANES,
    *HARDWARE_LANES,
    *CICD_LANES,
]

apply_operation_owners(LANES)

__all__ = ["LANES"]
