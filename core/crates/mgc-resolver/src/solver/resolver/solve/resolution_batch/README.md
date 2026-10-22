# Resolution batch / Materialize lô resolve

This owner records selected packages, asks the provider which dependencies to enqueue, and overlaps that work with version fetching for the next wave through the existing `join` call.

Owner này ghi nhận package được chọn, hỏi provider dependency nào cần enqueue, đồng thời chạy song song với việc tải version của wave sau qua `join` hiện có.

It preserves queue insertion order, the existing `add_resolution` owner, the `add_resolution` profile mark and next-wave prefetch error propagation. Runtime acceptance remains open.
