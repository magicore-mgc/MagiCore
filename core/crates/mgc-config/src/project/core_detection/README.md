# Project core detection / Nhận diện core project

`core_detection.rs` owns the stable `ProjectConfig` detection API and precedence: `.mgc.core`, `mgc.toml`, explicit declarations, then signatures. `manifest_declarations/` parses explicit MagiCore identity from supported manifests; `signatures/` owns conservative language/framework inference. Local errors preserve messages and sources at the public `anyhow` boundary.

`core_detection.rs` giữ API `ProjectConfig` ổn định và thứ tự ưu tiên: `.mgc.core`, `mgc.toml`, khai báo tường minh, rồi chữ ký. `manifest_declarations/` đọc identity MagiCore từ manifest được hỗ trợ; `signatures/` nhận diện ngôn ngữ/framework theo hướng bảo thủ. Lỗi cục bộ giữ nguyên nội dung và source chain tại biên `anyhow` public.
