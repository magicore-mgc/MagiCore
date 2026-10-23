# Cloud scaffold processor

## Purpose

Owns native cloud starters without registry template layers. `clo.rs` keeps the `CloProcessor` facade; `selection/` lists natively supported ids, `common/` holds the shared Node contract-test command, and each provider owns its files: `terraform/`, `cdk/` (CDK synth contract test), `cloudflare/`, `lambda/`, `pulumi/` (manifest contract test). Unknown ids still fall through to the Pulumi branch exactly as before.

## Phạm vi

Owner này sinh starter cloud native không cần template layer registry. `clo.rs` giữ facade `CloProcessor`; `selection/` liệt kê id hỗ trợ native, `common/` giữ lệnh test hợp đồng Node dùng chung, mỗi provider sở hữu file riêng: `terraform/`, `cdk/` (test hợp đồng synth), `cloudflare/`, `lambda/`, `pulumi/` (test hợp đồng manifest). Id lạ vẫn rơi vào nhánh Pulumi y như trước.
