# Registry user management

Owns user creation, deletion, and access-token revocation. Deletion and revocation validate the registry URL, use the bounded no-redirect admin client, encode dynamic path segments, and return diagnostics that do not expose credentials or the revoked token.

Quản lý tạo user, xóa user và thu hồi access token. Xóa/thu hồi xác thực URL registry, dùng admin client giới hạn thời gian và không redirect, encode path segment động, đồng thời không đưa credential hoặc token bị thu hồi vào lỗi.
