# GitHub Actions scanner owner

EN: Owns the public GitHub Actions audit routes. gh_actions.rs reads bounded workflow files and coordinates scanning/reporting; workflow_text/ owns the conservative YAML-subset policy checks; report/ maps findings to shared audit types; types.rs holds the stable public finding shape. It does not parse general YAML or change audit policy.

VI: Quản lý các route audit GitHub Actions công khai. gh_actions.rs đọc workflow theo giới hạn và điều phối quét/report; workflow_text/ giữ các kiểm tra policy trên tập con YAML bảo thủ; report/ đổi finding sang type audit dùng chung; types.rs giữ shape public ổn định. Folder này không parse YAML tổng quát và không đổi policy audit.
