# AI cache source map

The cache facade lives in `mod.rs` and exports owners only. Registry roots are mapped by `cache_dir/`; model identity-to-path validation is in `model_cache_path`; cache presence, listing and logical-size reporting have `is_cached/`, `list_cached_models/` and `cache_size/` owners; metadata persistence lives under `metadata/`; retention and explicit deletion are exposed through the `prune/` owners. The shared non-following filesystem size helper lives in `filesystem/`.

Facade cache nằm tại `mod.rs` và chỉ export các owner. Mapping root registry do `cache_dir/` quản lý; kiểm tra model ID và tạo path ở `model_cache_path`; cache presence, listing và báo logical-size có owner `is_cached/`, `list_cached_models/`, `cache_size/`; metadata persistence nằm trong `metadata/`; retention và xóa model được export qua owner `prune/`. Helper size filesystem không theo symlink dùng chung nằm ở `filesystem/`.

Destructive operations reject unsafe model IDs, skip cache-entry symlinks during prune, preserve source errors, and report logical bytes. Ambiguous `.meta.json` sidecars are retained rather than attributed to the wrong model. `Registry::Local` remains an explicit caller-selected cache root. Source tests are nearest to each changed owner; runtime verification remains pending.

Thao tác xóa từ chối model ID nguy hiểm, bỏ qua symlink entry khi prune, giữ source error và báo logical bytes. Sidecar `.meta.json` mơ hồ được giữ thay vì gán nhầm cho model khác. `Registry::Local` vẫn là cache root do caller chọn rõ. Test source đặt gần owner thay đổi; runtime verification đang chờ.
