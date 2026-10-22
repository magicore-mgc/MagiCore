"""State passed between release-gate validation owners."""

from __future__ import annotations

from dataclasses import dataclass, field


LaneKey = tuple[str, str, str]


@dataclass
class GateState:
    data: dict
    failures: list[str] = field(default_factory=list)
    release_scope: set[LaneKey] = field(default_factory=set)
    seen: set[LaneKey] = field(default_factory=set)
    release_blocking_seen: set[LaneKey] = field(default_factory=set)
    native_pm_scope: set[LaneKey] = field(default_factory=set)


@dataclass(frozen=True)
class LaneContext:
    lane: dict
    key: LaneKey
    label: str
    scope_out: bool
    source_required: tuple[str, ...] | None
