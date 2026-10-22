# Legacy signing / Ký legacy

This owner signs v2/v3 lock content with a legacy signature sidecar. It rejects incomplete classified-package provenance before changing signer metadata; the shared serializer repeats the gate immediately before writing. It refuses to route a v4 document through the legacy representation.

Owner này ký lock v2/v3 bằng sidecar chữ ký legacy. Nó từ chối provenance chưa đầy đủ của package đã phân loại trước khi đổi signer metadata; serializer dùng chung kiểm tra lại ngay trước lúc ghi. Owner từ chối route document v4 qua representation legacy.
