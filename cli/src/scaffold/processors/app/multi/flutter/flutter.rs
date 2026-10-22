//! Flutter output implementation.
//! Triển khai output Flutter.

use super::super::super::MultiProjectContext;
use anyhow::Result;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::app::implementation) fn write(
    context: &MultiProjectContext<'_>,
) -> Result<()> {
    let target = context.target;
    let slug = context.slug.as_str();

    // flutter/ — dart entry
    write_file(
        &target.join("flutter").join("pubspec.yaml"),
        &format!(
            "name: {}\ndescription: MagiCore Flutter platform entry\ndependencies:\n  flutter:\n    sdk: flutter\n",
            slug
        ),
    )?;
    write_file(
        &target.join("flutter").join("lib").join("main.dart"),
        "import 'package:flutter/material.dart';\n\nvoid main() => runApp(const App());\n\nclass App extends StatelessWidget {\n  const App({super.key});\n\n  @override\n  Widget build(BuildContext context) {\n    return MaterialApp(home: Scaffold(body: Center(child: Text('hello from MagiCore (flutter)'))));\n  }\n}\n",
    )?;
    Ok(())
}
