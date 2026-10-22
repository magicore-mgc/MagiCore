# .NET manifest detector

## Purpose

Owns the existing root .csproj/.sln predicate and creates the existing NuGet OSV ScanStep. The implementation lives in dotnet.rs; mod.rs only wires and re-exports the stable dotnet_step function.

## Phạm vi

Sở hữu predicate manifest root .csproj/.sln hiện có và tạo ScanStep NuGet OSV không đổi. Implementation nằm trong dotnet.rs; mod.rs chỉ wiring và export function dotnet_step ổn định.

This owner does not add manifest formats or alter scanner behavior.
