# Termination coordinator tests / Test coordinator termination

This folder covers the local 100-ms TERM-to-KILL coordination bound. Process-group identity and planning cases live with their owners. Real child cleanup remains covered by `mgc-exec` runner/process-scan tests; these tests must not signal the developer's own process.

Folder này kiểm tra giới hạn điều phối TERM-to-KILL 100 ms. Test identity và planner nằm cạnh owner tương ứng. Cleanup child thật vẫn do test runner/process-scan của `mgc-exec` bao phủ; test tại đây không được signal process của người chạy.
