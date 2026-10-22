# Flutter application container lane

`build_flutter_container.sh` scaffolds the Flutter app through MGC, resolves the project lock through the host MGC binary with isolated data/cache paths, verifies the pinned Flutter installer contract, and builds/runs the generated image.

`build_flutter_container.sh` scaffold app Flutter qua MGC, resolve lock dự án bằng binary MGC trên host với data/cache cô lập, xác minh hợp đồng installer Flutter đã ghim rồi build/chạy image được sinh.

Preserve the absence of direct `dart pub get`, the generated `mgc.lock` and `.mgc.core` checks, image name and Compose lifecycle. The workflow's Flutter setup remains the runtime owner.
