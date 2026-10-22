# cargo-audit JSON document

## Purpose

Owns the private serde document schema and extraction of the JSON object from cargo-audit output, including tolerated leading progress text.

## Phạm vi

Sở hữu schema serde nội bộ và việc lấy object JSON từ output cargo-audit, gồm cả progress text được phép đứng trước.

Unknown top-level metadata remains tolerated. Required nested fields still fail closed; unknown nested fields follow serde's existing tolerance.
