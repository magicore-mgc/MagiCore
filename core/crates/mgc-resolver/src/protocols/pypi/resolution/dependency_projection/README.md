# dependency_projection / dependency_projection

Projects Requires-Dist entries into package edges and lock markers. Invalid entries and requested dependency extras remain fail-closed; only certainly excluded environment markers are omitted.

Chiếu Requires-Dist thành cạnh package và lock marker. Entry sai và dependency extras được yêu cầu vẫn fail-closed; chỉ bỏ marker môi trường chắc chắn bị loại.

The owner preserves requirement order, marker order, the current unrequested-extra behavior, unsupported-extra diagnostics, and shared MgError types.

Owner giữ thứ tự requirement/marker, cách xử lý extra chưa yêu cầu, diagnostic unsupported extra và MgError dùng chung.
