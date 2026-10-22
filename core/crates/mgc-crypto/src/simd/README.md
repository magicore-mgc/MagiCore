# SIMD reporting / Báo cáo SIMD

Reports the CPU SIMD capability, retains the public capability value and renders the existing info string. Actual SIMD kernel selection remains inside the BLAKE3 dependency; the numeric multiplier is an unmeasured legacy estimate, not benchmark evidence.

Báo cáo capability SIMD của CPU, giữ value public và dựng chuỗi info hiện tại. Việc chọn kernel SIMD thực tế vẫn do dependency BLAKE3; multiplier số là ước lượng legacy chưa đo, không phải bằng chứng benchmark.

- `capability/` owns the public enum, names, display multipliers and serde representation. / `capability/` sở hữu enum public, tên, multiplier hiển thị và biểu diễn serde.
- `detection/` owns runtime architecture-gated feature detection. / `detection/` sở hữu phát hiện feature runtime có cfg theo kiến trúc.
- `info/` owns the public summary string. / `info/` sở hữu chuỗi tổng hợp public.
