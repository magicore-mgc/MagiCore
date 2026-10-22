# response

Owns fail-closed parsing of npm Bulk Advisory responses and local projection of advisories onto locked package versions. The parser rejects unrequested package names, wrong JSON shapes, incomplete records, invalid ranges, and invalid installed versions; valid advisories with no matching pin produce no finding.

Sở hữu parser fail-closed cho response npm Bulk Advisory và chiếu advisory lên version package đã lock. Parser từ chối tên package không được hỏi, JSON sai dạng, record thiếu field, range lỗi và version cài không hợp lệ; advisory hợp lệ nhưng không khớp pin thì không tạo finding.
