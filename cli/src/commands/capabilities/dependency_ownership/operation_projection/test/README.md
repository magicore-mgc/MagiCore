# Operation projection tests / Test projection operation

The test compares each serialized operation owner with the existing `dep_gate::owner_for` decision for the same full context and checks the special Cargo precondition. It verifies projection fidelity without establishing runtime package lifecycle support.

Test đối chiếu owner operation đã serialize với quyết định `dep_gate::owner_for` cho cùng context đầy đủ và kiểm tra điều kiện Cargo đặc biệt. Nó xác minh projection, không chứng minh lifecycle package runtime.
