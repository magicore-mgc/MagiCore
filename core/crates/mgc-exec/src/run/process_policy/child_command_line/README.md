# child_command_line

Owns the empty-native-argv guard for a monitored child and continues to use the shared ProcessPolicyError.

Sở hữu guard argv native rỗng cho child được monitor và tiếp tục dùng ProcessPolicyError chung.

The validator has an error result because an absent monitored command is a concrete inspection failure, unlike an ordinary matcher no-match. Focused tests live under test/.
