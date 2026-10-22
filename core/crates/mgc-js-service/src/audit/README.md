# audit

The public facade preserves the shared npm advisory API. Request construction, tarball-host trust, and response parsing live in separate owners so Web/App/Cloud integrations can share the same rules without depending on one adapter.

Facade công khai giữ nguyên API advisory npm dùng chung. Dựng request, trust host tarball và parse response nằm ở các owner riêng để tích hợp Web/App/Cloud có thể dùng cùng quy tắc mà không phụ thuộc adapter khác.
