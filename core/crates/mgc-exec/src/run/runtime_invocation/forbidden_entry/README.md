# forbidden_entry

Owns normalization and classification of a runtime-discovered script or module against the shared forbidden-tool set. It delegates token normalization to allowlist and preserves the special dotted Python module rule.

Sở hữu chuẩn hóa và phân loại script/module tìm được qua runtime theo danh sách tool bị cấm. Nó dùng normalize token từ allowlist và giữ quy tắc riêng cho Python module có hậu tố.

The result is optional because an unrecognized script is an ordinary non-match. This owner does not launch processes or make policy exceptions. Focused tests are in test/forbidden_entry.rs.
