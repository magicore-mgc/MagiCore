# Windows active scan

Owns the active Windows Job Object scan and maps its retry-controller outcomes into the wait-session contract.

Forbidden descendants and inspection failures fail closed. A child-exited result transfers the existing Job guard into the Windows root-exit owner.

# Windows scan khi process đang chạy

> Owner này quản lý Job Object scan khi command Windows đang chạy và ánh xạ kết quả retry controller sang contract phiên chờ.

> Child bị cấm và lỗi inspection vẫn fail-closed. Kết quả child đã thoát chuyển Job guard hiện tại sang owner Windows root-exit.
