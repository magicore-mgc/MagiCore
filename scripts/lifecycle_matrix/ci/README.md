# Lifecycle matrix CI reports

This folder owns presentation-only CI entry points for lifecycle evidence. `report_lane_verdicts.py` prints the generated schema version, each lane verdict and all recorded dimension statuses. Validation and release decisions belong to `validation/release_gate/`, not the log renderer.

## Tiếng Việt

Folder này sở hữu entry point chỉ phục vụ hiển thị log CI cho lifecycle evidence. `report_lane_verdicts.py` in schema version, verdict từng lane và toàn bộ trạng thái dimension. Quyết định validate/release thuộc `validation/release_gate/`, không thuộc renderer.

Add focused source-contract tests under the nearest `test/` owner during the later test phase. Running the report or gate is deferred until all production-source decomposition is complete.

Thêm test contract đặt gần owner trong `test/` ở pha test sau. Chưa chạy report hay gate cho đến khi hoàn tất tách production source toàn repository.
