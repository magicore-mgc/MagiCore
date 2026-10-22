# Age policy settings

Owns the typed release-age values loaded from the project and local store. Project settings are already converted from hours to seconds; persisted store settings remain seconds. The missing-timestamp escape hatch is carried with the same policy so the guard cannot silently drop it.

Sở hữu type tuổi phát hành đã nạp từ project và store cục bộ. Cấu hình project được đổi từ giờ sang giây; store vẫn dùng giây. Escape hatch cho timestamp thiếu đi cùng policy để guard không làm rơi cấu hình này.
