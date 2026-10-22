# Public server lifecycle and route table / Vòng đời server public và bảng route

Owns MgDevServer construction, watcher lifetime, shared state construction, ordered Axum routes, listener binding and serve lifecycle.

Sở hữu khởi tạo MgDevServer, vòng đời watcher, dựng state dùng chung, thứ tự Axum route, bind listener và serve.

The route order and public configuration/API stay stable. Request handlers remain in their focused owners.

Thứ tự route và cấu hình/API public được giữ nguyên. Handler request nằm trong owner trách nhiệm riêng.
