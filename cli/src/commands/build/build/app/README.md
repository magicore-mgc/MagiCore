# App build owner / Owner build App

## Responsibility / Trách nhiệm

Coordinates App build configuration, runtime selection and single- or multi-platform build execution. `configuration/` validates explicit language settings; `multi/` validates selected platform targets before invoking any tool.

Điều phối cấu hình build App, chọn runtime và thực thi build một/nhiều nền tảng. `configuration/` kiểm tra khai báo language tường minh; `multi/` xác nhận toàn bộ platform target trước khi gọi tool.

## Errors and tests / Lỗi và test

Configuration read/parse/language failures and invalid/incomplete multi-target results now use typed errors owned by their respective folders, preserving the prior diagnostic text and source where available. Shared missing-tool/no-artifact errors remain at the shared CLI boundary. Colocated tests are still deferred; test-only exports are gated by the `app` feature.

Lỗi đọc/parse/language cấu hình và platform multi-target sai/thiếu nay dùng error typed ở đúng folder, giữ diagnostic cũ và source khi có. Lỗi thiếu tool/không tạo artifact dùng chung vẫn ở boundary CLI. Test colocated còn chờ; test-only exports được gate theo feature `app`.
