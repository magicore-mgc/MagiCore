# Web release lifecycle owners

`web_lifecycle.sh` creates one temporary project and exports its directory and installed `mgc` path to four ordered phases: initializer/provenance plus install/test/build/audit, `dev` HTTP probe, `run` HTTP probe, and warm/offline reinstall. The dev/run probes source one shared process owner for HTTP checks and cross-platform child cleanup.

`web_lifecycle.sh` tạo một dự án tạm rồi truyền thư mục và đường dẫn `mgc` đã cài cho bốn pha theo thứ tự: initializer/provenance cùng install/test/build/audit, HTTP probe cho `dev`, HTTP probe cho `run`, và cài lại có cache/offline. Hai probe nạp chung một owner để kiểm tra HTTP và dọn process con trên nhiều OS.

The coordinator must keep one project root across all phases. A missing official create initializer marker, generator provenance, test script, build output, acceptable strict-audit result, real HTTP response, clean process shutdown, warm-cache evidence or offline install is a lane failure. The current source step calls `install --offline`; the presence of a populated cache is checked by the preceding warm-install log. A separate network-denial sentinel is not currently present and must be assessed in the later workflow/test verification phase.

Coordinator phải giữ cùng một root dự án qua mọi pha. Thiếu marker initializer create chính thức, provenance generator, test script, build output, kết quả audit strict hợp lệ, HTTP response thật, dừng process sạch, bằng chứng cache warm hoặc install offline thì lane phải fail. Source hiện gọi `install --offline`; log warm-install trước đó xác minh cache đã có dữ liệu. Chưa có sentinel chặn mạng riêng, cần đánh giá ở pha kiểm tra workflow/test sau.
