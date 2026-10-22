# shell

Owns argv parsing for sh, bash, dash, zsh, ksh and ash. It identifies only a shell file operand; -c command strings, -s stdin, and a lone dash remain opaque.

Sở hữu parse argv cho sh, bash, dash, zsh, ksh và ash. Parser chỉ nhận diện file operand; chuỗi -c, stdin -s và dấu gạch đơn luôn opaque.

Unknown syntax returns None rather than guessing a script. No shell is invoked here. Nearest tests are in test/shell.rs.
