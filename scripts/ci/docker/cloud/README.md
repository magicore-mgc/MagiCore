# Cloud CDK container lane

`build_cdk_container.sh` creates the TypeScript CDK project through MGC, installs its graph, validates the generated MGC/core/Dockerfile/Compose contract, then builds and runs the image.

`build_cdk_container.sh` tạo dự án CDK TypeScript qua MGC, cài dependency graph, xác minh hợp đồng MGC/core/Dockerfile/Compose được sinh rồi build và chạy image.

Keep the generated container's install/test/build assertions and Compose image reuse configuration aligned. Do not replace the MGC lifecycle with a direct ecosystem installer.
