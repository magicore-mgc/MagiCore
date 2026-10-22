# npm route owners / Các owner route npm

This module preserves the crate-facing `npm::routes` API and delegates route registration and handlers by npm registry responsibility. It owns package metadata, tarballs, dist-tags, account/token actions, search, scope canonicalization, batch publish and route assembly.

Module này giữ API `npm::routes` mà crate đang dùng, đồng thời tách đăng ký route và handler theo trách nhiệm registry npm: metadata package, tarball, dist-tag, tài khoản/token, search, chuẩn hóa scope, batch publish và ghép route.

The owner tree keeps the existing HTTP status, headers, response bodies, authentication/trusted-publishing checks, storage calls and route registration order. `StatusCode` remains the current request error contract. The nearest `src/test/npm.rs` file remains unchanged and is registered from this facade through a relative path.

Cây owner giữ nguyên HTTP status, header, response body, kiểm tra xác thực/trusted-publishing, lời gọi storage và thứ tự đăng ký route. `StatusCode` tiếp tục là hợp đồng lỗi hiện tại. File test gần nhất `src/test/npm.rs` không đổi nội dung và được đăng ký từ facade bằng relative path.

- `npm.rs`: stable route export.
- `routes/`: merge route families in the original order.
- `package/`, `tarball/`, `dist_tags/`, `user/`, `search/`, `batch_publish/`: independent route and handler families.
- `scope/`: canonical scoped-name construction shared by endpoint owners.
- Each behavior folder documents its module boundary in its own README.

- `npm.rs`: export route ổn định.
- `routes/`: ghép nhóm route theo thứ tự gốc.
- `package/`, `tarball/`, `dist_tags/`, `user/`, `search/`, `batch_publish/`: nhóm route và handler độc lập.
- `scope/`: chuẩn hóa tên package có scope, dùng chung giữa các owner.
- Mỗi folder có behavior mô tả ranh giới module trong README riêng.
