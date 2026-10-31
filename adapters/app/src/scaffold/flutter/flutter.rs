//! Built-in Flutter project template — project Flutter có sẵn trong sản phẩm.
//! Template project Flutter dựng sẵn (không cần SDK Flutter để scaffold).
//!
//! The app adapter owns a native pub.dev engine: `mgc install`, `mgc add` and
//! the lock replay never invoke the `flutter` binary. Only the scaffold used to
//! delegate to `flutter create`, which made `create-app flutter` fail on any
//! machine without the SDK. This template writes the same minimal, valid
//! project shape that the pub.dev lane can manage, so install works without the
//! SDK; `mgc build` / `mgc test` still need it and stay fail-closed with
//! guidance.
//! Adapter app có engine pub.dev native: `mgc install`, `mgc add` và replay
//! lock không gọi binary `flutter`. Chỉ scaffold từng ủy quyền `flutter create`,
//! khiến `create-app flutter` hỏng trên máy không có SDK. Template này ghi
//! đúng dạng project tối thiểu hợp lệ mà lane pub.dev quản lý được, nên install
//! chạy không cần SDK; `mgc build` / `mgc test` vẫn cần SDK và fail-closed kèm
//! hướng dẫn.

use std::path::Path;

use mgc_types::MgResult;

/// Flutter SDK constraint declared by generated projects.
/// Ràng buộc SDK Flutter mà project sinh ra khai báo.
pub const FLUTTER_SDK_CONSTRAINT: &str = ">=3.19.0";

/// Dart SDK constraint that ships with the declared Flutter floor.
/// Ràng buộc SDK Dart đi kèm mức Flutter tối thiểu đã khai báo.
pub const DART_SDK_CONSTRAINT: &str = ">=3.3.0 <4.0.0";

/// Write a minimal Flutter project for `project_name` into `target_dir`.
/// Ghi project Flutter tối thiểu cho `project_name` vào `target_dir`.
///
/// The destination must not exist yet: a create never merges into an existing
/// tree, and the caller reports the same conflict on its other source paths.
/// Đích chưa được tồn tại: create không hòa vào cây đã có, và caller báo cùng
/// lỗi xung đột ở các đường nguồn khác.
pub async fn scaffold_project(project_name: &str, target_dir: &Path) -> MgResult<()> {
    if target_dir.try_exists()? {
        return Err(unsupported(format!(
            "Directory '{}' already exists",
            target_dir.display()
        )));
    }
    // A Dart package name must be a valid identifier; derive it once so the
    // pubspec, the imports and the lockfile agree on one name.
    let package = package_name(project_name);
    std::fs::create_dir_all(target_dir.join("lib"))?;
    std::fs::create_dir_all(target_dir.join("test"))?;

    let files: [(&str, String); 6] = [
        ("pubspec.yaml", pubspec(&package)),
        ("analysis_options.yaml", analysis_options().to_string()),
        ("lib/main.dart", main_dart()),
        ("test/widget_test.dart", widget_test()),
        (".gitignore", gitignore().to_string()),
        ("README.md", readme(project_name, &package)),
    ];
    for (relative, contents) in files {
        write_regular_file(&target_dir.join(relative), &contents)?;
    }
    Ok(())
}

/// Derive a valid Dart package name from a project name.
/// Suy tên package Dart hợp lệ từ tên project.
///
/// Dart package names are `lowercase_with_underscores`; a hyphen or a capital
/// makes `flutter pub get` reject the manifest, so the name is normalised once
/// here instead of letting a later build fail on it.
/// (Tên package Dart là `lowercase_with_underscores`; gạch nối hay chữ hoa làm
/// `flutter pub get` từ chối manifest, nên chuẩn hóa một lần ở đây.)
pub fn package_name(project_name: &str) -> String {
    let mut normalized = String::with_capacity(project_name.len());
    for character in project_name.chars() {
        if character.is_ascii_alphanumeric() {
            normalized.push(character.to_ascii_lowercase());
        } else if !normalized.ends_with('_') {
            normalized.push('_');
        }
    }
    let trimmed = normalized.trim_matches('_');
    if trimmed.is_empty() {
        return "app".to_string();
    }
    // A package name must start with a letter or underscore.
    // (Tên package phải bắt đầu bằng chữ cái hoặc gạch dưới.)
    if trimmed.starts_with(|character: char| character.is_ascii_digit()) {
        return format!("app_{trimmed}");
    }
    trimmed.to_string()
}

/// Render `pubspec.yaml` with the Flutter SDK dependency and no pub.dev deps.
/// Render `pubspec.yaml` với dependency SDK Flutter và không dependency pub.dev.
///
/// Every declared dependency is a pub.dev package, never a `sdk:` package.
///
/// A `sdk: flutter` dependency needs the SDK's own `pubspec.yaml` closure to
/// expand (see `manifest/flutter/sdk`), so declaring one would make `mgc
/// install` fail on exactly the SDK-less machine this template exists for.
/// `cupertino_icons` and `flutter_lints` keep the manifest a real pub.dev
/// resolution instead of a project with nothing to install.
/// (Mọi dependency khai báo đều là package pub.dev, không bao giờ `sdk:`.
/// Dependency `sdk: flutter` cần closure `pubspec.yaml` của SDK để mở rộng, nên
/// khai báo nó sẽ làm `mgc install` hỏng đúng trên máy không có SDK — lý do
/// template này tồn tại. `cupertino_icons` và `flutter_lints` giữ manifest là
/// một lần resolve pub.dev thật thay vì project không có gì để cài.)
fn pubspec(package: &str) -> String {
    format!(
        "name: {package}\n\
         description: A Flutter app scaffolded by MagiCore.\n\
         publish_to: 'none'\n\
         version: 0.1.0\n\
         \n\
         environment:\n\
         \x20 sdk: '{DART_SDK_CONSTRAINT}'\n\
         \x20 flutter: '{FLUTTER_SDK_CONSTRAINT}'\n\
         \n\
         dependencies:\n\
         \x20 cupertino_icons: ^1.0.8\n\
         \n\
         dev_dependencies:\n\
         \x20 flutter_lints: ^5.0.0\n\
         \n\
         flutter:\n\
         \x20 uses-material-design: true\n"
    )
}

/// Render the analyzer configuration Flutter templates ship.
/// Render cấu hình analyzer mà template Flutter phát hành.
fn analysis_options() -> &'static str {
    "include: package:flutter_lints/flutter.yaml\n\
     \n\
     linter:\n\
     \x20 rules:\n\
     \x20   prefer_const_constructors: true\n"
}

/// Render `lib/main.dart` with a real `MaterialApp` entry point.
/// Render `lib/main.dart` với entry point `MaterialApp` thật.
fn main_dart() -> String {
    "// Entry point — điểm vào của ứng dụng.\n\
     import 'package:flutter/material.dart';\n\
     \n\
     void main() => runApp(const MagiCoreApp());\n\
     \n\
     /// Root widget — widget gốc của ứng dụng.\n\
     class MagiCoreApp extends StatelessWidget {\n\
     \x20 const MagiCoreApp({super.key});\n\
     \n\
     \x20 @override\n\
     \x20 Widget build(BuildContext context) {\n\
     \x20   return MaterialApp(\n\
     \x20     title: 'MagiCore App',\n\
     \x20     theme: ThemeData(colorSchemeSeed: Colors.indigo, useMaterial3: true),\n\
     \x20     home: const Scaffold(\n\
     \x20       body: Center(child: Text('Hello from MagiCore')),\n\
     \x20     ),\n\
     \x20   );\n\
     \x20 }\n\
     }\n"
    .to_string()
}

/// Render a smoke test that boots the app widget.
/// Render smoke test khởi động widget app.
fn widget_test() -> String {
    "// Smoke test — test khói cho widget gốc.\n\
     import 'package:flutter/material.dart';\n\
     import 'package:flutter_test/flutter_test.dart';\n\
     \n\
     import '../lib/main.dart';\n\
     \n\
     void main() {\n\
     \x20 testWidgets('renders the app shell', (WidgetTester tester) async {\n\
     \x20   await tester.pumpWidget(const MagiCoreApp());\n\
     \x20   expect(find.byType(MaterialApp), findsOneWidget);\n\
     \x20 });\n\
     }\n"
    .to_string()
}

/// Render `.gitignore` for Flutter/Dart and MagiCore local state.
/// Render `.gitignore` cho Flutter/Dart và trạng thái cục bộ MagiCore.
fn gitignore() -> &'static str {
    "# Flutter/Dart build output — output build Flutter/Dart\n\
     .dart_tool/\n\
     build/\n\
     .flutter-plugins\n\
     .flutter-plugins-dependencies\n\
     .packages\n\
     \n\
     # MagiCore local state — trạng thái cục bộ của MagiCore\n\
     .magicore/\n"
}

/// Render the project README with the SDK boundary spelled out.
/// Render README project nêu rõ ranh giới SDK.
fn readme(project_name: &str, package: &str) -> String {
    format!(
        "# {project_name}\n\
         \n\
         Flutter app scaffolded by MagiCore (package `{package}`).\n\
         \n\
         - `mgc install` resolves `pubspec.yaml` against pub.dev with MagiCore's native engine (the Flutter SDK is not needed).\n\
         - `mgc build` and `mgc test` run the Flutter SDK; without it they fail closed with install guidance.\n\
         - `mgc dev` starts the app through the Flutter SDK for the same reason.\n\
         - If the SDK is installed later, `flutter create .` adds the platform folders (android/, ios/, web/).\n"
    )
}

/// Write one regular file, creating parents and refusing a symlink destination.
/// Ghi một file thường, tạo thư mục cha và từ chối đích là symlink.
fn write_regular_file(path: &Path, contents: &str) -> MgResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    // A pre-existing symlink would redirect the write outside the project tree.
    // (Symlink sẵn có sẽ chuyển hướng ghi ra ngoài cây project.)
    if std::fs::symlink_metadata(path)
        .map(|metadata| metadata.file_type().is_symlink())
        .unwrap_or(false)
    {
        return Err(unsupported(format!(
            "refusing to write scaffold file '{}': destination is a symlink",
            path.display()
        )));
    }
    std::fs::write(path, contents)?;
    Ok(())
}

/// Typed error for the offline Flutter template.
/// Lỗi typed cho template Flutter offline.
fn unsupported(guidance: String) -> mgc_types::MgError {
    mgc_types::MgError::Unsupported {
        core: "app",
        capability: "scaffold",
        guidance,
    }
}
