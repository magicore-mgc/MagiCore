# Web install owner

This folder keeps the stable public `install` command and orders its implementation stages. `options/` owns immutable CLI input; `preflight/` verifies root/offline constraints; `backend_install/` routes supported non-JS projects to their language adapter; `plan/` resolves the Web adapter, validates targets, applies dedupe policy and performs root package mutation; `monorepo_install/` and `single_target_install/` execute their respective lanes. The install file is only the coordinator.

The stage ordering is safety-sensitive: preflight and backend routing happen before the Web plan; mixed-stack members are checked before manifest/lock/store mutation; root adds pass through the shared gateway; target execution starts only once the plan succeeds. Keep tests in their current location during the source-only pass, then colocate them after all repository source structures are complete.

# Owner install Web

Folder này giữ lệnh `install` công khai ổn định và sắp thứ tự các bước implementation. `options/` sở hữu input CLI bất biến; `preflight/` xác minh root/offline; `backend_install/` định tuyến project non-JS được hỗ trợ sang adapter ngôn ngữ; `plan/` resolve Web adapter, xác minh target, áp chính sách dedupe và mutation package root; `monorepo_install/` và `single_target_install/` thực thi hai lane. File install chỉ làm điều phối.

Thứ tự bước có ý nghĩa an toàn: preflight và backend routing trước Web plan; xác minh mixed-stack trước mutation manifest/lock/store; package root đi qua shared gateway; chỉ thực thi target sau khi plan thành công. Trong lượt source-only, test giữ nguyên chỗ; sẽ chuyển test gần owner sau khi hoàn tất cấu trúc source toàn repository.
