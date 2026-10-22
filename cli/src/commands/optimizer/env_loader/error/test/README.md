# Optimizer environment diagnostic tests

## Purpose

Checks that malformed env errors expose a line number without echoing the key/value, and that local typed errors retain legacy display text and their concrete type.

## Phạm vi

Kiểm tra lỗi env sai định dạng chỉ lộ số dòng, không in key/value; đồng thời đối chiếu message legacy và downcast của lỗi typed cục bộ.
