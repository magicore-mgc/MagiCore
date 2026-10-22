# Lock reuse

Reuses a prior lock graph for delta installs when policy allows. If minimum-release-age requires fresh registry timestamps, this route returns `Ok(false)` so the caller resolves the complete graph.

Tái sử dụng lock graph cũ cho delta install khi policy cho phép. Nếu minimum-release-age cần timestamp registry mới, route trả `Ok(false)` để caller resolve toàn graph.

Its nearest test pins that fallback without changing package-add behavior.

Test gần owner ghim fallback này mà không đổi hành vi package-add.
