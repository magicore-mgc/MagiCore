# Shared HTTP TLS owners

EN: This module preserves the mgc-http TLS API while separating configuration, rustls trust setup, PEM material, reqwest application, request headers, and registry URL checks. The custom verifier remains dangerous and the current opt-in branch is preserved.

VI: Module này giữ API TLS của mgc-http, tách cấu hình, trust rustls, PEM, áp dụng cho reqwest, header request và kiểm tra URL registry. Verifier tùy chỉnh vẫn nguy hiểm và nhánh opt-in hiện tại được giữ nguyên.
