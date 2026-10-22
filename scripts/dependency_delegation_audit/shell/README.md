# Shell scanner / Scanner shell

`primitives.py` owns lexical helpers, `commands.py` owns command segmentation and substitution, `aliases.py` resolves static aliases, `alias_mutation.py` invalidates aliases after writes, `execution.py` checks interpreter/process boundaries, `traps.py` validates trap callbacks, and `shell.py` integrates them. The scanner stays conservative and preserves existing unparsed/blocking results.

`primitives.py` xử lý lexical helper; `commands.py` tách command/substitution; `aliases.py` phân giải alias tĩnh; `execution.py` kiểm tra interpreter/process boundary; `scanner.py` nối các phần. Giữ kết quả conservative và fail-closed hiện hành.
