# windows_cmd

Owns Windows cmd /c and /k operand extraction and the minimal quote-aware splitter. It returns only the first executable plus its separated arguments, preserving the policy that the remaining command text is opaque.

Sở hữu tách operand của Windows cmd /c, /k và bộ tách tối thiểu có xét quote. Module chỉ trả executable đầu và các argument đã tách, giữ chính sách không dò phần command text còn lại.

This code does not spawn cmd and introduces no OS side effects. No-match is returned as None. Nearest tests are in test/windows_cmd.rs.
