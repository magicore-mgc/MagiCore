# dart_pub

Owns recognition of the Dart/Flutter pub operand after runtime flags. It does not decide whether the following subcommand mutates dependencies; that remains the shared policy owner's job.

Sở hữu nhận diện operand pub của Dart/Flutter sau runtime flag. Module không quyết định subcommand tiếp theo có sửa dependency hay không; việc đó thuộc policy dùng chung.

The parser returns None for another positional command or an unsupported runtime. Nearest tests are in test/dart_pub.rs.
