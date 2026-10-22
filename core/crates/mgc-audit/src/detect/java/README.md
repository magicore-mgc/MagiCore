# Java manifest detector

## Purpose

Owns the existing build.gradle/build.gradle.kts/pom.xml predicate and creates the existing Maven OSV ScanStep. The implementation lives in java.rs; mod.rs only wires and re-exports the stable java_step function.

## Phạm vi

Sở hữu predicate manifest build.gradle/build.gradle.kts/pom.xml hiện có và tạo ScanStep Maven OSV không đổi. Implementation nằm trong java.rs; mod.rs chỉ wiring và export function java_step ổn định.

This owner does not add manifest formats or alter scanner behavior.
