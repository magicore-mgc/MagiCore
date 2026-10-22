# Library manifest detection / Nhận diện manifest thư viện

## Responsibility / Trách nhiệm

Owns focused discovery of root-level SDK-style `*.csproj` files and Java build descriptors. Reads only regular manifest files through the shared project-safety helper.

Sở hữu dò `*.csproj` SDK-style ở root và descriptor build Java. Chỉ đọc manifest file thường qua helper an toàn project dùng chung.

## Errors and tests / Lỗi và test

Filesystem and unsafe-manifest errors propagate from the shared reader. Detection itself defines no separate public error domain. Unit coverage will be moved here in the later test pass; no executable checks ran now.

Lỗi filesystem và manifest không an toàn được propagate từ reader dùng chung. Bản thân nhận diện chưa có error domain public riêng. Unit test sẽ chuyển về đây ở pha test sau; hiện chưa chạy kiểm tra thực thi.
