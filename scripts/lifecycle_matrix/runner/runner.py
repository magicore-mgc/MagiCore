"""Lifecycle lane orchestration."""

from __future__ import annotations

from .context import LaneRunContext
from .preflight import toolchain_preflight
from .scaffold import scaffold_phase
from .preparation import preparation_phase
from .steps import steps_phase
from .run_probe import run_probe_phase
from .dev_probe import dev_probe_phase
from .install_evidence import install_evidence_phase
from .optimizer import optimizer_phase
from .outcomes import outcomes_phase

def run_lane(mgc_bin: str, lane: dict) -> dict:
    """Execute one lane's ordered lifecycle and collect evidence."""
    ctx = LaneRunContext.create(mgc_bin, lane)
    if not toolchain_preflight(ctx):
        return ctx.finish(toolchain_available=False)
    scaffold_phase(ctx)
    preparation_phase(ctx)
    steps_phase(ctx)
    run_probe_phase(ctx)
    dev_probe_phase(ctx)
    install_evidence_phase(ctx)
    optimizer_phase(ctx)
    outcomes_phase(ctx)
    return ctx.finish(toolchain_available=True)

__all__ = ["run_lane"]
