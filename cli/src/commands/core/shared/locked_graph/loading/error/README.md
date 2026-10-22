# Locked graph loading errors

This owner types read, parse, policy, project-owner and graph-conversion failures. It preserves the current English diagnostic at the `anyhow` CLI boundary and keeps native source errors available for inspection. Recovery classification distinguishes transient read retry, lock repair, project-core correction and file-access correction. It is advisory: this module performs no retry; any future automatic retry must stay inside the loading owner.

Owner này gắn kiểu cho lỗi đọc, phân tích, policy, cấu hình core và chuyển graph. Nó giữ nguyên thông báo tiếng Anh tại biên CLI và lưu lỗi nguồn gốc để kiểm tra. Phân loại recovery tách retry đọc tạm thời, sửa lock, chỉnh core project và sửa quyền truy cập file. Đây chỉ là gợi ý: module không tự retry; nếu sau này có retry tự động thì chỉ owner loading được thực hiện.
