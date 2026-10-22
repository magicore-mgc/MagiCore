"""Shared lifecycle environment preparation."""

from __future__ import annotations

from .swift import (
    swift_toolchain_environment,
)
import os

def lifecycle_environment(sandbox: str, project_path: str) -> dict[str, str]:
    """Keep MagiCore and common toolchain write paths inside each lane.
    (Giới hạn đường ghi MagiCore và cache toolchain phổ biến trong sandbox.)"""
    env = os.environ.copy()
    host_home = env.get("HOME") or os.path.expanduser("~")
    home = os.path.join(sandbox, ".home")
    cache = os.path.join(sandbox, ".cache")
    data = os.path.join(sandbox, ".data")
    temp = os.path.join(sandbox, ".tmp")
    cargo_home = os.path.join(sandbox, ".cargo-home")
    go_path = os.path.join(sandbox, ".go")
    env["HOME"] = home
    env["USERPROFILE"] = home
    # Windows dirs::home_dir can ignore HOME/USERPROFILE; pin the shared store.
    # Windows dirs::home_dir có thể bỏ qua HOME/USERPROFILE; ghim store riêng.
    env["MAGICORE_STORE_ROOT"] = os.path.join(home, ".magicore", "store")
    env["TMPDIR"] = temp
    env["TMP"] = temp
    env["TEMP"] = temp
    env["XDG_CACHE_HOME"] = cache
    env["XDG_CONFIG_HOME"] = os.path.join(home, ".config")
    env["XDG_DATA_HOME"] = data
    env["XDG_STATE_HOME"] = os.path.join(home, ".local", "state")
    env["PYTHONPYCACHEPREFIX"] = os.path.join(cache, "python-bytecode")
    env["CARGO_HOME"] = cargo_home
    env["CARGO_TARGET_DIR"] = os.path.join(sandbox, ".cargo-target")
    # Rustup's proxy binaries need the installed toolchain metadata even when
    # CARGO_HOME is lane-isolated. Keep that host toolchain read-only; never
    # fall back to downloading/installing a toolchain during matrix execution.
    # (Proxy rustup cần metadata toolchain đã cài dù CARGO_HOME được cô lập.
    # Chỉ đọc toolchain host; không tải/cài toolchain trong lúc chạy matrix.)
    rustup_home = env.get("RUSTUP_HOME")
    if not rustup_home or not os.path.isdir(rustup_home):
        rustup_home = os.path.join(host_home, ".rustup")
    if os.path.isdir(rustup_home):
        env["RUSTUP_HOME"] = rustup_home
    else:
        env.pop("RUSTUP_HOME", None)
    env["RUSTUP_AUTO_INSTALL"] = "0"
    env["PIP_CACHE_DIR"] = os.path.join(cache, "pip")
    env["UV_CACHE_DIR"] = os.path.join(cache, "uv")
    env["npm_config_cache"] = os.path.join(cache, "npm")
    env["DENO_DIR"] = os.path.join(cache, "deno")
    env["BUN_INSTALL_CACHE_DIR"] = os.path.join(cache, "bun")
    env["GOPATH"] = go_path
    env["GOMODCACHE"] = os.path.join(go_path, "pkg", "mod")
    env["GOCACHE"] = os.path.join(cache, "go-build")
    env["GRADLE_USER_HOME"] = os.path.join(cache, "gradle")
    env["DOTNET_CLI_HOME"] = os.path.join(home, ".dotnet")
    env["ANDROID_USER_HOME"] = os.path.join(home, ".android")
    env["PUB_CACHE"] = os.path.join(cache, "pub")
    env["NUGET_PACKAGES"] = os.path.join(cache, "nuget")
    env["SWIFT_MODULECACHE_PATH"] = os.path.join(cache, "swift-modules")
    env["MGC_CACHE_DIR"] = os.path.join(project_path, ".magicore")
    # Several tools (notably Go) require TMPDIR to exist before startup.
    # Create only lane-owned tool/cache directories; project state remains
    # owned by the real `mgc` commands under test.
    # (Một số tool, đặc biệt Go, cần TMPDIR tồn tại trước khi khởi chạy.
    # Chỉ tạo thư mục tool/cache thuộc lane; project state để mgc quản lý.)
    for key in (
        "HOME",
        "XDG_CACHE_HOME",
        "XDG_CONFIG_HOME",
        "XDG_DATA_HOME",
        "XDG_STATE_HOME",
        "PYTHONPYCACHEPREFIX",
        "CARGO_HOME",
        "CARGO_TARGET_DIR",
        "MAGICORE_STORE_ROOT",
        "PIP_CACHE_DIR",
        "UV_CACHE_DIR",
        "npm_config_cache",
        "DENO_DIR",
        "BUN_INSTALL_CACHE_DIR",
        "GOPATH",
        "GOMODCACHE",
        "GOCACHE",
        "GRADLE_USER_HOME",
        "DOTNET_CLI_HOME",
        "ANDROID_USER_HOME",
        "PUB_CACHE",
        "NUGET_PACKAGES",
        "SWIFT_MODULECACHE_PATH",
        "TMPDIR",
    ):
        os.makedirs(env[key], exist_ok=True)
    return env

def lifecycle_step_environment(
    lane: dict,
    step: str,
    lane_env: dict[str, str],
    flutter_sdk_pub_cache: str,
    flutter_sdk_home: str = "",
) -> dict[str, str]:
    """Use the warmed job-local Flutter SDK home/cache for test/build only.
    (Chỉ dùng HOME/cache Flutter SDK tạm của job cho test/build.)"""
    lane_env = swift_toolchain_environment(lane, step, lane_env)
    if (lane.get("core"), lane.get("language")) == ("app", "flutter") and step in {
        "test",
        "build",
    }:
        if not flutter_sdk_home:
            # Local/unwarmed runs keep the lane-private HOME and Pub cache.
            # Local/chưa warm thì giữ HOME và Pub cache riêng của lane.
            return lane_env
        # MgC project dependencies remain under its sandbox-owned store; this
        # home/cache matches the pre-warm step so Flutter does not bootstrap
        # its own SDK packages from a different HOME; it is not project resolution.
        # HOME/cache trùng bước warm để Flutter không bootstrap package SDK từ
        # HOME khác; chúng không dùng để resolve dependency của project.
        step_env = lane_env.copy()
        step_env["PUB_CACHE"] = flutter_sdk_pub_cache
        if flutter_sdk_home:
            step_env["HOME"] = flutter_sdk_home
            step_env["USERPROFILE"] = flutter_sdk_home
        return step_env
    return lane_env

__all__ = ['lifecycle_environment', 'lifecycle_step_environment']
