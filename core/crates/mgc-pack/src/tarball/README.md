# Tarball API / API tarball

This folder owns deterministic package archive creation. Its root module preserves mgc_pack::tarball::{pack, PackResult}; implementation is separated into coordination, entry writing, digesting, staging and local errors.

Thư mục này quản lý tạo package archive xác định. Module gốc giữ mgc_pack::tarball::{pack, PackResult}; implementation được tách thành điều phối, ghi entry, hash, staging và lỗi cục bộ.

Input leaves are opened through mgc-platform without following a final symlink or reparse point. The platform helper does not pin parent-directory handles; replacement of parent path components remains a documented boundary. Public integration coverage stays at tests/tarball.rs.

Leaf input được mở qua mgc-platform, không đi theo symlink/reparse ở thành phần cuối. Helper nền tảng không ghim handle thư mục cha; việc thay component thư mục cha vẫn là ranh giới được ghi nhận. Integration test công khai tiếp tục ở tests/tarball.rs.
