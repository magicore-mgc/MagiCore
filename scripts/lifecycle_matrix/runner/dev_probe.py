"""HTTP/HMR/rebuild dev probe and process-tree cleanup."""

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

def dev_probe_phase(ctx):
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

    # `dev` probe (P0-mới-1, Tech Lead vòng-6/7): spawn `mgc dev`, wait
    # for bind + HTTP 200, EDIT a source file, expect NEW output
    # (rebuild), check the HMR websocket endpoint responds, then kill the
    # tree + sweep the port. Reuses the same kill-tree/sweep discipline
    # as the run probe. A dev server that only prints help (the old
    # step) can never pass this.
    # Probe `dev` (P0-mới-1): spawn `mgc dev`, chờ bind + HTTP 200, SỬA
    # file source, chờ output MỚI (rebuild), kiểm tra endpoint websocket
    # HMR phản hồi, rồi kill cây + quét port. Tái dùng đúng kỷ luật
    # kill-tree/sweep như probe run. Dev server chỉ in help (bước cũ)
    # không bao giờ pass được.
    devp = lane.get("dev_probe")
    if devp is not None and dims.get("create") == STATUS_NATIVE:
        import urllib.request
        port = devp["port"]
        project_path = os.path.join(sandbox, project_dir)
        try:
            dev_proc = subprocess.Popen(
                [mgc_bin, "dev"],
                cwd=project_path,
                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
                env=lane_env,
                start_new_session=(os.name != "nt"),
            )
        except FileNotFoundError:
            _fail(f"cannot spawn mgc for dev probe: {mgc_bin}")

        dev_served = False
        hmr_ok = False
        rebuilt = False
        deadline = time.monotonic() + min(timeout_s, 120)

        def _dev_kill_tree():
            # Same discipline as the run probe: group kill, then port
            # sweep for re-parented listeners.
            # Cùng kỷ luật probe run: kill group, rồi quét port cho
            # listener bị re-parent.
            if os.name != "nt":
                import signal
                try:
                    os.killpg(os.getpgid(dev_proc.pid), signal.SIGTERM)
                except (ProcessLookupError, PermissionError):
                    dev_proc.terminate()
            else:
                run_text_capture(
                    ["taskkill", "/T", "/F", "/PID", str(dev_proc.pid)],
                    capture_output=True,
                )
                dev_proc.terminate()

        def _dev_sweep_port():
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

        output_before_edit = ""
        while time.monotonic() < deadline:
            if dev_proc.poll() is not None:
                out = (dev_proc.stdout.read() if dev_proc.stdout else "") or ""
                dims["dev"] = STATUS_FAILED
                detail["dev_output"] = ("dev probe: server exited early — " + out)[-2000:]
                break
            for host in ("127.0.0.1", "localhost"):
                try:
                    with urllib.request.urlopen(
                        f"http://{host}:{port}/", timeout=2
                    ) as resp:
                        if resp.status == 200:
                            dev_served = True
                            break
                except Exception:
                    pass
            if dev_served:
                # HMR endpoint must respond (101 upgrade or 400/405 —
                # anything but connection-refused proves the WS route is
                # wired; a plain GET to a WS route cannot complete the
                # handshake but MUST be accepted by the server).
                # Endpoint HMR phải phản hồi (101 upgrade hoặc 400/405 —
                # bất cứ gì khác connection-refused chứng minh route WS
                # được wire; GET thường tới route WS không hoàn tất
                # handshake nhưng server PHẢI chấp nhận).
                for hmr_host in ("127.0.0.1", "localhost"):
                    try:
                        urllib.request.urlopen(
                            f"http://{hmr_host}:{port}{devp['hmr_path']}", timeout=2
                        )
                        hmr_ok = True
                    except urllib.error.HTTPError:
                        # 4xx from the route itself = the endpoint EXISTS.
                        # (4xx từ chính route = endpoint TỒN TẠI.)
                        hmr_ok = True
                    except Exception:
                        pass
                    if hmr_ok:
                        break
                break
            time.sleep(1)

        if dev_served and dev_proc.poll() is None:
            # Drain output so far, then edit the source — the dimension
            # demands a REBUILD, not just a boot.
            # Xả output đến giờ, rồi sửa source — dimension đòi REBUILD,
            # không chỉ khởi động.)
            import threading
            output_chunks: list[str] = []

            def _reader():
                try:
                    for line in dev_proc.stdout:
                        output_chunks.append(line)
                except Exception:
                    pass

            reader = threading.Thread(target=_reader, daemon=True)
            reader.start()
            output_before_edit = "".join(output_chunks)
            src = os.path.join(project_path, devp["source_file"])
            if os.path.isfile(src):
                with open(src, "a", encoding="utf-8") as f:
                    f.write(devp["source_edit"])
            rebuild_deadline = time.monotonic() + 30
            while time.monotonic() < rebuild_deadline:
                # New output after the edit = the watcher saw the change
                # and recompiled/reloaded.
                # (Output mới sau khi sửa = watcher thấy thay đổi và
                # biên dịch lại/reload.)
                if "".join(output_chunks) != output_before_edit:
                    rebuilt = True
                    break
                if dev_proc.poll() is not None:
                    break
                time.sleep(1)
            reader.join(timeout=2)

        if dev_proc.poll() is None:
            _dev_kill_tree()
            try:
                dev_proc.wait(timeout=10)
            except subprocess.TimeoutExpired:
                _dev_kill_tree()
                try:
                    dev_proc.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    dev_proc.kill()
        _dev_sweep_port()

        if dims.get("dev") != STATUS_FAILED:
            if dev_served and rebuilt and hmr_ok:
                dims["dev"] = lifecycle_pass_status(lane, "dev")
            else:
                dims["dev"] = STATUS_FAILED
                detail["dev_output"] = (
                    f"dev probe: served={dev_served} hmr_endpoint={hmr_ok} "
                    f"rebuilt_after_edit={rebuilt} — the dimension requires ALL three"
                )

__all__ = ['dev_probe_phase']
