# Tests / Kiểm thử

Focused tests for public execution-type defaults live here. Keep tests for process spawn, denial, cancellation and timeout in the owning execution-policy/platform test modules.

Test gần owner cho default của type public ở đây. Test spawn, deny, cancel và timeout thuộc module policy/platform tương ứng.

`mod.rs` only includes `types.rs` so the test module keeps its original parent scope.
`mod.rs` chỉ include `types.rs` để giữ nguyên scope cha của test module.
