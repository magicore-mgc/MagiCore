//! Flutter implementation.
//! Triển khai Flutter.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::{slugify, write_file};

pub(in crate::scaffold::processors::app::implementation) fn write(
    target: &Path,
    name: &str,
) -> Result<()> {
    write_file(
        &target.join("pubspec.yaml"),
        &format!(
            "name: {}\ndescription: MagiCore Flutter app\npublish_to: none\nversion: 0.1.0+1\nenvironment:\n  sdk: '>=3.4.0 <4.0.0'\ndependencies:\n  flutter:\n    sdk: flutter\ndev_dependencies:\n  flutter_test:\n    sdk: flutter\nflutter:\n  uses-material-design: true\n",
            slugify(name).replace('-', "_")
        ),
    )?;
    write_file(
        &target.join("lib").join("main.dart"),
        "import 'package:flutter/material.dart';\n\nvoid main() => runApp(const MagiCoreApp());\n\nclass MagiCoreApp extends StatelessWidget {\n  const MagiCoreApp({super.key});\n\n  @override\n  Widget build(BuildContext context) {\n    return const MaterialApp(\n      home: Scaffold(body: Center(child: Text('MagiCore Flutter app'))),\n    );\n  }\n}\n",
    )?;
    write_file(
        &target.join("test").join("widget_test.dart"),
        "import 'package:flutter_test/flutter_test.dart';\n\nimport '../lib/main.dart';\n\nvoid main() {\n  testWidgets('renders the MagiCore app', (tester) async {\n    await tester.pumpWidget(const MagiCoreApp());\n    expect(find.text('MagiCore Flutter app'), findsOneWidget);\n  });\n}\n",
    )?;
    // Flutter web platform dir (P0 finding, 2026-09-12):
    // `flutter build web` requires web/ to exist — without
    // it the build exits 2 ("does not have a web platform
    // directory"). The scaffold ships the minimal standard
    // web entry so create→install→test→build(web) runs on
    // plain CI runners (no Android SDK / Xcode needed).
    // Thư mục nền tảng web Flutter (P0 finding): `flutter
    // build web` yêu cầu web/ tồn tại — thiếu nó build exit
    // 2. Scaffold mang web entry chuẩn tối thiểu để
    // create→install→test→build(web) chạy trên runner CI
    // thường (không cần Android SDK / Xcode).
    write_file(
        &target.join("web").join("index.html"),
        &format!(
            "<!DOCTYPE html>\n<html>\n<head>\n  <base href=\"$FLUTTER_BASE_HREF\">\n  <meta charset=\"UTF-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n  <title>{name}</title>\n</head>\n<body>\n  <script src=\"flutter_bootstrap.js\" async></script>\n</body>\n</html>\n"
        ),
    )?;
    write_file(
        &target.join("web").join("manifest.json"),
        &format!(
            "{{\n  \"name\": \"{name}\",\n  \"short_name\": \"{name}\",\n  \"start_url\": \"/\",\n  \"display\": \"standalone\",\n  \"background_color\": \"#ffffff\",\n  \"theme_color\": \"#ffffff\",\n  \"description\": \"A MagiCore Flutter app\"\n}}\n"
        ),
    )?;
    Ok(())
}
