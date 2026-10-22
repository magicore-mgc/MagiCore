# Language dispatch / Chuyển lane ngôn ngữ

## Responsibility / Trách nhiệm

Owns the exhaustive AppLanguage match and delegates each existing lane; unsupported languages still return the same explicit error.

## Tests / Kiểm thử

Component tests stay next to platform/command/config owners. The dispatch branch matrix is not fully covered by E2E yet; the SwiftPM E2E covers only its lane.

## Tiếng Việt

Owner này chứa match đầy đủ trên `AppLanguage` và chuyển tiếp từng lane đang có; ngôn ngữ chưa hỗ trợ vẫn trả lỗi tường minh cũ. Ma trận các nhánh dispatch chưa được E2E đầy đủ; E2E SwiftPM chỉ chứng minh lane SwiftPM.
