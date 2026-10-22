# Registry user URL

EN: Maps the shared registry URL contract to the user-registration endpoint while preserving registry path prefixes and username encoding.

VI: Owner này áp dụng contract URL registry dùng chung cho endpoint tạo user, giữ prefix registry và encode username.

Remote HTTP, user-info, query and fragment remain rejected through the shared URL owner; loopback HTTP remains available for local development. The original URL tests were moved here unchanged.

HTTP từ xa, user-info, query và fragment tiếp tục bị chặn bởi shared URL owner; loopback HTTP vẫn dùng được khi phát triển local. Test URL gốc được chuyển tới đây không đổi.
