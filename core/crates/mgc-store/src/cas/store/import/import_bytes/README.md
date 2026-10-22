# Byte import owner

Owns single byte imports, executable-mode imports, parallel batch preparation, rebuildable batches and declared-hash import entry points. Shared hash-checked persistence remains in write_bytes. It keeps caller-facing ContentStore methods on the same type and uses the shared StoreError.

Phụ trách import byte đơn, import theo executable mode, chuẩn bị batch song song, batch rebuildable và entry point nhận hash khai báo. Ghi bền vững có kiểm hash nằm ở write_bytes. Giữ method ContentStore trên cùng type và dùng StoreError chung.

Nearest source regression test: test/import_bytes.rs. Bulk/security behavior remains covered by the frozen adversarial_integrity integration suite.
