# Hardware core owner

This folder is available in every CLI feature build because the package names were historically unconditional. It owns Hardware package names and the template materialization flow shared with Game optimizer setup. Processor-generated `optimizer` and `bench` remain registry-free; other framework layers must pass `ensure_layer`, with the existing no-fallback behavior. Materialization remains idempotent for an existing target. `shared.rs` re-exports the old names for internal compatibility.

Only an existing real directory is treated as an idempotent materialization. Symlinks and non-directories at the target path fail closed. Resolver failures retain their typed diagnostic and source instead of being mislabeled as a missing layer. `error.rs` owns typed target-inspection, layer-resolution, missing-layer and scaffold failures. Focused helper/error test sources live in `test/`; tests and builds remain NOT_RUN during the source-first pass.

# Owner của Hardware core

Folder này có mặt trong mọi feature build của CLI vì các package name vốn luôn được export. Nó sở hữu tên package Hardware và luồng materialize dùng chung với Game optimizer. `optimizer` và `bench` do processor sinh nên tiếp tục không qua registry; framework khác phải qua `ensure_layer` và giữ nguyên hành vi không fallback. Target đã tồn tại vẫn được giữ nguyên. `shared.rs` export lại tên cũ để tương thích nội bộ.

Chỉ thư mục thật đã tồn tại mới được xem là materialize idempotent. Symlink và file khác tại target bị chặn. Lỗi resolver giữ diagnostic và source typed, không bị gắn nhầm thành thiếu layer. `error.rs` sở hữu lỗi kiểm tra target, resolve layer, thiếu layer và scaffold. Source test helper/error đặt trong `test/`; test và build còn NOT_RUN trong phase source-first.
