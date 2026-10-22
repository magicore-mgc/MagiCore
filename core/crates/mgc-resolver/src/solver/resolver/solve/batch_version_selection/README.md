# Batch version selection / Chọn version theo lô

This owner fetches missing version lists and applies the existing override, already-resolved, and fresh-selection phases in their original order. It owns the `batch_prefetch_versions` and `select_versions` profile marks.

Owner này tải trước danh sách version còn thiếu và giữ nguyên thứ tự xử lý override, package đã resolve, rồi package mới. Owner sở hữu các profile mark `batch_prefetch_versions` và `select_versions`.

All parse/provider/constraint errors continue through the existing `SolveError` contract. Existing integration tests remain at the resolver crate target; runtime verification is deferred.
