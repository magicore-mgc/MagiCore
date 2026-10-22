"""HTTP-backed production run probe and process-tree cleanup."""

from __future__ import annotations
from ..contract.ownership import (
    lifecycle_pass_status,
)
from ..contract.status import (
    STATUS_FAILED,
    STATUS_NATIVE,
)
from ..errors import (
    _fail,
)
from ..process.process import (
    run_text_capture,
)
from .context import (
    LaneRunContext,
)
import os
import subprocess
import time

def run_probe_phase(ctx):
    lane = ctx.lane
    mgc_bin = ctx.mgc_bin
    timeout_s = ctx.timeout_s
    sandbox = ctx.sandbox
    dims = ctx.dims
    detail = ctx.detail
    project_dir = ctx.project_dir
    project_path = ctx.project_path
    lane_env = ctx.lane_env
    flutter_sdk_pub_cache = ctx.flutter_sdk_pub_cache
    flutter_sdk_home = ctx.flutter_sdk_home
    run_step = ctx.run_step

    # `run` probe (P0 finding #6): long-lived production entry — start
    # `mgc run <script>`, wait for a REAL HTTP roundtrip on the probe
    # port, then shut down cleanly. A server that only starts (or dies
    # instantly) is a FAILED run, never a passed one.
    # Probe `run` (P0 finding #6): entry production dài hạn — khởi động
    # `mgc run <script>`, chờ HTTP roundtrip THẬT trên port probe, rồi
    # tắt sạch. Server chỉ khởi động (hoặc chết ngay) là run FAILED,
    # không bao giờ passed.
    probe = lane.get("run_probe")
    if probe is not None and dims.get("create") == STATUS_NATIVE:
        import urllib.request
        port = probe["port"]
        argv = ["run", probe["script"]]
        # New session/process group: killing the probe must kill the
        # WHOLE tree (mgc → node → vite) — a leaked vite child held the
        # port and broke later lanes/tests (zombie caught in REVIEW
        # round 2, 2026-09-12).
        # Session/process-group riêng: kill probe phải giết CẢ CÂY
        # (mgc → node → vite) — vite con sót lại giữ port và phá lane/
        # test sau đó (zombie bắt được ở REVIEW vòng 2).
        try:
            proc = subprocess.Popen(
                [mgc_bin] + argv,
                cwd=os.path.join(sandbox, project_dir),
                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
                env=lane_env,
                start_new_session=(os.name != "nt"),
            )
        except FileNotFoundError:
            _fail(f"cannot spawn mgc for run probe: {mgc_bin}")

        def _kill_tree(p):
            # POSIX: signal the whole process group (mgc + node + vite).
            # Windows: taskkill /T kills the tree; p.kill() is fallback.
            # POSIX: tín hiệu tới cả process group (mgc + node + vite).
            # Windows: taskkill /T giết cả cây; p.kill() là phương án dự phòng.
            if os.name != "nt":
                import signal
                try:
                    os.killpg(os.getpgid(p.pid), signal.SIGTERM)
                except (ProcessLookupError, PermissionError):
                    p.terminate()
            else:
                run_text_capture(
                    ["taskkill", "/T", "/F", "/PID", str(p.pid)],
                    capture_output=True,
                )
                p.terminate()

        def _sweep_port_leak():
            # The dev toolchain (node/vite) can end up RE-PARENTED into
            # its own session (observed: PPID=1, own PGID) — group kill
            # misses it. Sweep by PORT: find the PID still LISTENING on
            # the probe port and kill it directly. A leaked listener
            # breaks every later lane and test on this machine (REVIEW
            # round 2 finding, 2026-09-12).
            # Toolchain dev (node/vite) có thể bị RE-PARENT thành session
            # riêng (quan sát: PPID=1, PGID riêng) — kill group không trúng.
            # Quét theo PORT: tìm PID còn LISTEN trên port probe và giết
            # trực tiếp. Listener sót làm vỡ mọi lane/test sau đó.
            if os.name != "nt":
                try:
                    out = run_text_capture(
                        ["lsof", "-ti", f"tcp:{port}", "-sTCP:LISTEN"],
                        capture_output=True, text=True, timeout=10,
                    ).stdout.strip()
                    for pid_s in out.split():
                        if pid_s.isdigit():
                            try:
                                os.kill(int(pid_s), 9)
                            except (ProcessLookupError, PermissionError):
                                pass
                except (FileNotFoundError, subprocess.TimeoutExpired):
                    pass

        served = False
        deadline = time.monotonic() + min(timeout_s, 90)
        while time.monotonic() < deadline:
            if proc.poll() is not None:
                # Server exited before ever serving — a real failure.
                # Server thoát trước khi phục vụ — fail thật.
                out = (proc.stdout.read() if proc.stdout else "") or ""
                dims["run"] = STATUS_FAILED
                detail["run_output"] = out[-2000:]
                break
            # Probe BOTH stacks: vite listens on ::1 (IPv6 localhost)
            # on some hosts and 127.0.0.1 on others — the roundtrip must
            # not depend on which stack the dev server picked.
            # Probe CẢ HAI stack: vite listen trên ::1 (IPv6 localhost)
            # ở một số máy và 127.0.0.1 ở máy khác — roundtrip không
            # được phụ thuộc stack mà dev server chọn.
            for host in ("127.0.0.1", "localhost"):
                try:
                    with urllib.request.urlopen(
                        f"http://{host}:{port}/", timeout=2
                    ) as resp:
                        if resp.status == 200:
                            served = True
                            break
                except Exception:
                    pass
            if served:
                break
            time.sleep(1)
        if proc.poll() is None:
            _kill_tree(proc)
            try:
                proc.wait(timeout=10)
            except subprocess.TimeoutExpired:
                _kill_tree(proc)
                try:
                    proc.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    proc.kill()
        # Port sweep: the re-parented listener must be gone or the lane
        # leaks the port (breaks later lanes AND unrelated tests that
        # bind 4315, e.g. registry-server tests).
        # Quét port: listener bị re-parent phải biến mất, nếu không lane
        # rò port (phá lane sau và cả test không liên quan bind 4315,
        # vd test registry-server).
        _sweep_port_leak()
        if dims.get("run") != STATUS_FAILED:
            dims["run"] = lifecycle_pass_status(lane, "run") if served else STATUS_FAILED
            if not served:
                detail["run_output"] = (
                    "run probe: no HTTP 200 on localhost:%d before timeout" % port
                )

__all__ = ['run_probe_phase']
