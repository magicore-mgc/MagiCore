"""Mutable state for one isolated lifecycle lane run."""

from __future__ import annotations

from dataclasses import dataclass, field
import os
import shutil
import subprocess
import tempfile

from ..diagnostics.diagnostics import command_output_excerpt
from ..environment.flutter import flutter_sdk_home_directory, flutter_sdk_pub_cache_directory
from ..environment.pulumi import pulumi_local_environment
from ..environment.shared import lifecycle_environment, lifecycle_step_environment
from ..process.process import run_text_capture
from ..errors import _fail

STEP_TIMEOUT_DEFAULT_S = 600

@dataclass
class LaneRunContext:
    mgc_bin: str
    lane: dict
    timeout_s: int
    sandbox: str
    dims: dict = field(default_factory=dict)
    detail: dict = field(default_factory=dict)
    project_dir: str = ""
    project_path: str = ""
    lane_env: dict = field(default_factory=dict)
    flutter_sdk_pub_cache: str = ""
    flutter_sdk_home: str = ""
    blocked_lifecycle_steps: dict = field(default_factory=dict)
    install_status: str | None = None

    @classmethod
    def create(cls, mgc_bin: str, lane: dict) -> "LaneRunContext":
        timeout_s = int(os.environ.get("MGC_LIFECYCLE_STEP_TIMEOUT", STEP_TIMEOUT_DEFAULT_S))
        sandbox = tempfile.mkdtemp(prefix=f"mgc-lc-{lane['core']}-{lane['language']}-")
        project_dir = lane.get("project_dir", lane["scaffold"][-1])
        project_path = os.path.join(sandbox, project_dir)
        lane_env = lifecycle_environment(sandbox, project_path)
        flutter_sdk_pub_cache = flutter_sdk_pub_cache_directory(os.environ)
        flutter_sdk_home = flutter_sdk_home_directory(os.environ)
        if (lane.get("core"), lane.get("language")) == ("clo", "pulumi"):
            lane_env = pulumi_local_environment(sandbox, lane_env)
        return cls(
            mgc_bin=mgc_bin,
            lane=lane,
            timeout_s=timeout_s,
            sandbox=sandbox,
            detail={"sandbox": sandbox},
            project_dir=project_dir,
            project_path=project_path,
            lane_env=lane_env,
            flutter_sdk_pub_cache=flutter_sdk_pub_cache,
            flutter_sdk_home=flutter_sdk_home,
        )

    def run_step(self, argv: list[str], subdir: str = "", lifecycle_step: str = "") -> tuple[int, str]:
        cwd = os.path.join(self.sandbox, subdir) if subdir else self.sandbox
        step_env = (
            lifecycle_step_environment(
                self.lane, lifecycle_step, self.lane_env,
                self.flutter_sdk_pub_cache, self.flutter_sdk_home,
            )
            if lifecycle_step else self.lane_env
        )
        try:
            proc = run_text_capture(
                [self.mgc_bin] + argv, capture_output=True, text=True,
                cwd=cwd, env=step_env, timeout=self.timeout_s,
            )
            return proc.returncode, (proc.stdout or "") + (proc.stderr or "")
        except subprocess.TimeoutExpired:
            _fail(f"step timed out after {self.timeout_s}s: mgc {' '.join(argv)}")

    def finish(self, toolchain_available: bool = True) -> dict:
        shutil.rmtree(self.sandbox, ignore_errors=True)
        return {
            "dims": self.dims,
            "detail": self.detail,
            "toolchain_available": toolchain_available,
        }

__all__ = ["LaneRunContext", "STEP_TIMEOUT_DEFAULT_S"]
