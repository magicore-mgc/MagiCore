# Byte matcher

This private owner checks whether a binary slice contains a signature. Pickle scanning uses it while streaming chunks. It owns no format policy and exports only to the parent scanner module.

Owner riêng tư này kiểm tra một lát byte nhị phân có chứa signature hay không. Pickle scanner dùng nó khi đọc theo chunk. Owner không quyết định chính sách định dạng và chỉ export cho module scanner cha.

The matcher is intentionally small and has its unit cases in test/. Empty-pattern behavior follows slice::windows and is not used by current signatures.

Logic matcher được giữ nhỏ; test đơn vị nằm trong test/. Trường hợp pattern rỗng theo hành vi slice::windows và không được các signature hiện tại sử dụng.
