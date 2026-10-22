# Native outdated query / Query outdated native

Uses the selected adapter resolver for a fresh candidate graph, while requiring a usable current lock graph and preserving direct-dependency-only comparisons.

Dùng resolver của adapter được chọn để resolve graph candidate mới, đồng thời yêu cầu lock graph hiện tại dùng được và chỉ so sánh dependency trực tiếp.

The orchestrator's nearest test covers only the no-dependency early return; resolver-backed non-empty behavior is not claimed by that test.

Test gần owner orchestration chỉ phủ nhánh return sớm khi không có dependency; test này không đại diện hành vi resolve có dependency.
