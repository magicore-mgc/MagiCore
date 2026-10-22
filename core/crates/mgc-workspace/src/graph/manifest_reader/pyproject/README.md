# Python pyproject parser / Bộ phân tích pyproject của Python

Reads the existing `[project].name` field and leaves dependency maps empty. Invalid/unrecognized TOML continues to return `None`; file-read errors continue to propagate through the local manifest error.
