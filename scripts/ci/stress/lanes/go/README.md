# Go readonly execution lane / Lane chạy Go chỉ đọc

`verify_readonly_execution.sh` discovers the real Go SDK binary, exposes it to the E2Es and runs the execution and resolver cold-cache readonly probes. The workflow supplies the runner OS and a ten-minute timeout.

`verify_readonly_execution.sh` tìm binary Go SDK thật, truyền vào E2E và chạy probe readonly của execution/resolver với cache compiler lạnh. Workflow cung cấp OS runner và timeout mười phút.
