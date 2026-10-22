# Per-core common adapter tests

These tests exercise all current `CommonCommands` variants and verify that the dispatch adapter preserves their values. They call only the mapper; they do not execute registry, model, filesystem, publish or other command handlers.

## Vietnamese

Các test này đi qua toàn bộ variant `CommonCommands` hiện tại và xác minh adapter giữ nguyên giá trị khi dispatch. Test chỉ gọi mapper; không chạy registry, model, filesystem, publish hoặc command handler khác.
