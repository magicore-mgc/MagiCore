# Manifest conversion / Chuyển đổi manifest

Owns the `Manifest` ↔ `package.json` projection. It coordinates schema IO and workspace-range policy but does not own the JSON schema or atomic filesystem primitives.

Sở hữu projection `Manifest` ↔ `package.json`. Module điều phối IO schema và policy workspace range, không sở hữu JSON schema hay primitive filesystem atomic.

The public `parse_manifest` and `write_manifest` signatures remain exported from `mgc_js_service::manifest`. Keep unknown package fields and existing workspace ranges; fail if a workspace dependency would be replaced by a registry pin.

Chữ ký công khai `parse_manifest` và `write_manifest` vẫn được export từ `mgc_js_service::manifest`. Giữ field package chưa biết và workspace range hiện có; báo lỗi nếu dependency workspace bị thay bằng registry pin.
