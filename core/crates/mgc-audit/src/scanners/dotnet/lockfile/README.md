# NuGet lockfile reader

Parses the `dependencies` map in `packages.lock.json`, uses `resolved` before legacy `version`, and reports pins skipped for missing versions.

Parse map `dependencies` trong `packages.lock.json`, ưu tiên `resolved` hơn `version` cũ, và báo rõ pin bị bỏ qua vì thiếu version.
