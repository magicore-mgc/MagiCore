# Dependency ownership projection / Projection dependency ownership

## Responsibility / Trách nhiệm

Maps each record to ecosystem, manifest format, engine core and the per-operation owner from dep_gate. It does not grant capabilities or run operations.

Ánh xạ record sang ecosystem, manifest format, engine core và owner theo operation từ dep_gate. Module không cấp capability hay chạy operation.

## Errors and tests / Lỗi và test

It does not catch shared policy failures; catalog tests remain deferred.

Module không catch lỗi policy dùng chung; test catalog được hoãn.
