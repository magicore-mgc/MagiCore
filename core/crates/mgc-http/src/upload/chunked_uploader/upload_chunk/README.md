# Chunk upload / Upload chunk

Owns PATCH requests, Content-Range generation, server response status handling, and the current Range fallback parsing.

Quản lý request PATCH, dựng Content-Range, xử lý status và cách parse Range fallback hiện có.

The source currently trusts the returned Range shape and its fallback addition is unchecked. The resume cursor interpretation needs a behavior-level protocol fix and focused tests.

Source hiện tin vào cấu trúc Range trả về và phép cộng fallback chưa kiểm tra tràn. Cách diễn giải cursor resume cần được sửa và kiểm thử theo protocol.
