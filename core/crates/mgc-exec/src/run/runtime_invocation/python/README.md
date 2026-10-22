# python

Owns Python script and module operand parsing. It recognizes interpreter selection flags and the -m module form while treating every -c source form as opaque.

Sở hữu parse script/module operand của Python. Parser nhận các interpreter flag được hỗ trợ và dạng -m module, đồng thời coi mọi source dạng -c là opaque.

No-match is an expected Option result, including inline code; it is not an error. Module-to-manager classification belongs to forbidden_entry. Nearest tests are in test/python.rs.
