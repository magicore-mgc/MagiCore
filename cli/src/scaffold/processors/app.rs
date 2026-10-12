//! App scaffold fallbacks for supported framework lanes.

use std::path::Path;

use anyhow::Result;

use super::{slugify, write_file};

pub struct AppProcessor;

impl AppProcessor {
    pub fn files(target: &Path, name: &str, framework: &str) -> Result<()> {
        match framework {
            "kotlin" => {
                write_file(
                    &target.join("settings.gradle.kts"),
                    &format!("rootProject.name = \"{}\"\n", name),
                )?;
                write_file(
                    &target
                        .join("app")
                        .join("src")
                        .join("main")
                        .join("kotlin")
                        .join("Main.kt"),
                    "fun main() {\n    println(\"MagiCore Kotlin app scaffold\")\n}\n",
                )?;
            }
            "swift" => {
                write_file(
                    &target.join("Package.swift"),
                    &format!(
                        "// swift-tools-version: 5.9\nimport PackageDescription\n\nlet package = Package(\n    name: \"{}\",\n    targets: [\n        .executableTarget(name: \"MagiCoreApp\"),\n        .testTarget(name: \"MagiCoreAppTests\", dependencies: [\"MagiCoreApp\"])\n    ]\n)\n",
                        name
                    ),
                )?;
                write_file(
                    &target.join("Sources").join("MagiCoreApp").join("App.swift"),
                    "public func greeting() -> String {\n    \"MagiCore Swift app scaffold\"\n}\n\n@main\nstruct MagiCoreApp {\n    static func main() {\n        print(greeting())\n    }\n}\n",
                )?;
                write_file(
                    &target
                        .join("Tests")
                        .join("MagiCoreAppTests")
                        .join("AppTests.swift"),
                    "import XCTest\n@testable import MagiCoreApp\n\nfinal class AppTests: XCTestCase {\n    func testGreeting() {\n        XCTAssertEqual(greeting(), \"MagiCore Swift app scaffold\")\n    }\n}\n",
                )?;
            }
            "maui" => {
                let slug = slugify(name);
                write_file(
                    &target.join(format!("{slug}.csproj")),
                    "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup>\n    <TargetFrameworks>net8.0-android;net8.0-ios;net8.0-maccatalyst</TargetFrameworks>\n    <OutputType>Exe</OutputType>\n    <RootNamespace>MagiCoreApp</RootNamespace>\n    <UseMaui>true</UseMaui>\n    <SingleProject>true</SingleProject>\n    <ImplicitUsings>enable</ImplicitUsings>\n    <Nullable>enable</Nullable>\n  </PropertyGroup>\n</Project>\n",
                )?;
                write_file(
                    &target.join("MauiProgram.cs"),
                    "using Microsoft.Maui;\nusing Microsoft.Maui.Hosting;\n\nnamespace MagiCoreApp;\n\npublic static class MauiProgram\n{\n    public static MauiApp CreateMauiApp() => MauiApp.CreateBuilder()\n        .UseMauiApp<App>()\n        .Build();\n}\n",
                )?;
                write_file(
                    &target.join("App.xaml"),
                    "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n<Application xmlns=\"http://schemas.microsoft.com/dotnet/2021/maui\"\n             xmlns:x=\"http://schemas.microsoft.com/winfx/2009/xaml\"\n             x:Class=\"MagiCoreApp.App\">\n  <Application.Resources />\n</Application>\n",
                )?;
                write_file(
                    &target.join("App.xaml.cs"),
                    "namespace MagiCoreApp;\n\npublic partial class App : Application\n{\n    public App()\n    {\n        InitializeComponent();\n        MainPage = new MainPage();\n    }\n}\n",
                )?;
                write_file(
                    &target.join("MainPage.xaml"),
                    "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n<ContentPage xmlns=\"http://schemas.microsoft.com/dotnet/2021/maui\"\n             xmlns:x=\"http://schemas.microsoft.com/winfx/2009/xaml\"\n             x:Class=\"MagiCoreApp.MainPage\">\n  <VerticalStackLayout Padding=\"24\">\n    <Label Text=\"MagiCore MAUI app\" FontSize=\"24\" />\n  </VerticalStackLayout>\n</ContentPage>\n",
                )?;
                write_file(
                    &target.join("MainPage.xaml.cs"),
                    "namespace MagiCoreApp;\n\npublic partial class MainPage : ContentPage\n{\n    public MainPage() => InitializeComponent();\n}\n",
                )?;
            }
            "tauri" => {
                let slug = slugify(name);
                let identifier = format!("dev.magicore.{}", slug.replace('-', "."));
                let tauri_config = serde_json::json!({
                    "productName": name,
                    "version": "0.1.0",
                    "identifier": identifier,
                    "build": {
                        "devUrl": "http://localhost:1420",
                        "frontendDist": "../dist"
                    },
                    "app": {
                        "windows": [{ "title": name, "width": 800, "height": 600 }],
                        "security": { "csp": null }
                    },
                    "bundle": { "active": true, "targets": "all" }
                });
                write_file(
                    &target.join("package.json"),
                    &format!(
                        "{{\n  \"name\": \"{slug}\",\n  \"private\": true,\n  \"version\": \"0.1.0\",\n  \"scripts\": {{ \"build\": \"vite build\", \"dev\": \"vite --host 127.0.0.1 --port 1420 --strictPort\" }},\n  \"devDependencies\": {{ \"vite\": \"^6.0.0\" }}\n}}\n"
                    ),
                )?;
                write_file(
                    &target.join("index.html"),
                    "<!doctype html>\n<html><head><meta charset=\"UTF-8\"><title>MagiCore Tauri</title></head><body><main id=\"app\"></main><script type=\"module\" src=\"/src/main.js\"></script></body></html>\n",
                )?;
                write_file(
                    &target.join("src/main.js"),
                    "document.querySelector('#app').innerHTML = '<h1>MagiCore Tauri app</h1><p>Frontend scaffold ready.</p>';\n",
                )?;
                write_file(
                    &target.join("dist/index.html"),
                    "<!doctype html>\n<html><head><meta charset=\"UTF-8\"><title>MagiCore Tauri</title></head><body><main><h1>MagiCore Tauri app</h1></main></body></html>\n",
                )?;
                write_file(
                    &target.join("src-tauri/Cargo.toml"),
                    &format!(
                        "[package]\nname = \"{slug}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[build-dependencies]\ntauri-build = {{ version = \"2\" }}\n\n[dependencies]\ntauri = {{ version = \"2\" }}\n\n[features]\ncustom-protocol = [\"tauri/custom-protocol\"]\n"
                    ),
                )?;
                write_file(
                    &target.join("src-tauri/build.rs"),
                    "fn main() { tauri_build::build() }\n",
                )?;
                write_file(
                    &target.join("src-tauri/src/main.rs"),
                    "#![cfg_attr(not(debug_assertions), windows_subsystem = \"windows\")]\n\nfn main() {\n    tauri::Builder::default()\n        .run(tauri::generate_context!())\n        .expect(\"failed to run Tauri application\");\n}\n",
                )?;
                write_file(
                    &target.join("src-tauri/tauri.conf.json"),
                    &format!("{}\n", serde_json::to_string_pretty(&tauri_config)?),
                )?;
            }
            "react-native" => {
                let slug = slugify(name);
                let package = serde_json::json!({
                    "name": slug,
                    "version": "0.1.0",
                    "private": true,
                    "main": "index.js",
                    "scripts": {
                        "start": "expo start",
                        "android": "expo start --android",
                        "ios": "expo start --ios",
                        "web": "expo start --web"
                    },
                    "dependencies": {
                        "expo": "~57.0.27",
                        "expo-status-bar": "~57.0.1",
                        "react": "19.2.3",
                        "react-native": "0.86.3"
                    }
                });
                let app_config = serde_json::json!({
                    "expo": {
                        "name": name,
                        "slug": slug,
                        "version": "0.1.0",
                        "orientation": "portrait",
                        "ios": { "supportsTablet": true }
                    }
                });
                write_file(
                    &target.join("package.json"),
                    &format!("{}\n", serde_json::to_string_pretty(&package)?),
                )?;
                write_file(
                    &target.join("app.json"),
                    &format!("{}\n", serde_json::to_string_pretty(&app_config)?),
                )?;
                write_file(
                    &target.join("index.js"),
                    "import { registerRootComponent } from 'expo';\nimport App from './App';\n\nregisterRootComponent(App);\n",
                )?;
                write_file(
                    &target.join("App.js"),
                    "import { StatusBar } from 'expo-status-bar';\nimport { StyleSheet, Text, View } from 'react-native';\n\nexport default function App() {\n  return (\n    <View style={styles.container}>\n      <Text style={styles.title}>MagiCore React Native app</Text>\n      <Text>Expo project scaffold is ready.</Text>\n      <StatusBar style=\"auto\" />\n    </View>\n  );\n}\n\nconst styles = StyleSheet.create({\n  container: { flex: 1, backgroundColor: '#fff', alignItems: 'center', justifyContent: 'center', padding: 24 },\n  title: { fontSize: 24, fontWeight: '600', marginBottom: 8 },\n});\n",
                )?;
            }
            "flutter" => {
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
            }
            other => anyhow::bail!(
                "unsupported app framework '{other}'; supported values are flutter, kotlin, swift, react-native, maui, and tauri (multi is handled by the app scaffold command)"
            ),
        }

        Ok(())
    }

    pub fn files_multi(target: &Path, name: &str) -> Result<()> {
        let slug = slugify(name);
        let pkg = slug.replace('-', "_");

        // shared/ — Kotlin KMP commonMain + androidMain (nơi chứa logic dùng chung)
        write_file(
            &target.join("shared").join("build.gradle.kts"),
            &format!(
                "plugins {{\n    kotlin(\"multiplatform\") version \"2.0.0\"\n}}\n\nkotlin {{\n    androidTarget()\n    iosX64()\n    iosArm64()\n    iosSimulatorArm64()\n\n    sourceSets {{\n        commonMain.dependencies {{\n            implementation(\"org.jetbrains.kotlinx:kotlinx-coroutines-core:1.9.0\")\n        }}\n    }}\n\n    listOf(\n        iosX64(),\n        iosArm64(),\n        iosSimulatorArm64()\n    ).forEach {{ iosTarget ->\n        iosTarget.binaries.framework {{\n            baseName = \"{name}\"\n            isStatic = true\n        }}\n    }}\n}}\n\nandroid {{\n    namespace = \"{pkg}.shared\"\n    compileSdk = 34\n}}\n"
            ),
        )?;
        write_file(
            &target.join("shared").join("settings.gradle.kts"),
            &format!("rootProject.name = \"{}-shared\"\n", slug),
        )?;
        write_file(
            &target
                .join("shared")
                .join("src")
                .join("commonMain")
                .join("kotlin")
                .join(pkg.as_str())
                .join("Shared.kt"),
            "package {pkg}\n\nobject Shared {\n    fun hello(): String = \"hello from MagiCore shared (KMP)\"\n}\n",
        )?;

        // android/ — kotlin entry dùng shared (gradle include + app module phụ thuộc shared)
        write_file(
            &target.join("android").join("settings.gradle.kts"),
            &format!(
                "rootProject.name = \"{}-android\"\n\ninclude(\":app\", \":shared\")\n\nproject(\":shared\").projectDir = file(\"../shared\")\n",
                slug
            ),
        )?;
        write_file(
            &target.join("android").join("build.gradle.kts"),
            "plugins {\n    id(\"com.android.application\") version \"8.5.0\" apply false\n    kotlin(\"android\") version \"2.0.0\" apply false\n}\n",
        )?;
        write_file(
            &target.join("android").join("app").join("build.gradle.kts"),
            &format!(
                "plugins {{\n    id(\"com.android.application\")\n    kotlin(\"android\")\n}}\n\nandroid {{\n    namespace = \"{pkg}.android\"\n    compileSdk = 34\n    defaultConfig {{\n        applicationId = \"{pkg}.android\"\n        minSdk = 24\n    }}\n}}\n\ndependencies {{\n    implementation(project(\":shared\"))\n}}\n"
            ),
        )?;
        write_file(
            &target
                .join("android")
                .join("app")
                .join("src")
                .join("main")
                .join("kotlin")
                .join("Main.kt"),
            "package {pkg}.android\n\nimport {pkg}.Shared\n\nfun main() {\n    println(Shared.hello())\n}\n",
        )?;

        // ios/ — swift + objc entry, cùng SPM target
        write_file(
            &target.join("ios").join("Package.swift"),
            &format!(
                "// swift-tools-version: 5.9\nimport PackageDescription\n\nlet package = Package(\n    name: \"{}\",\n    targets: [.executableTarget(name: \"{}\")]\n)\n",
                slug, slug
            ),
        )?;
        write_file(
            &target
                .join("ios")
                .join("Sources")
                .join(slug.as_str())
                .join("main.swift"),
            "print(\"MagiCore iOS app scaffold (swift)\")\n",
        )?;
        // ObjC bridge nằm ngoài SPM Sources (Xcode native project dùng) — SPM không chấp mixed language.
        write_file(
            &target.join("ios").join("ObjcBridge").join("ObjcBridge.h"),
            "#import <Foundation/Foundation.h>\n\n@interface MGShared : NSObject\n+ (NSString *)hello;\n@end\n",
        )?;
        write_file(
            &target.join("ios").join("ObjcBridge").join("ObjcBridge.m"),
            "#import \"ObjcBridge.h\"\n\n@implementation MGShared\n+ (NSString *)hello {\n    return @\"hello from MagiCore shared (objc)\";\n}\n@end\n",
        )?;

        // react-native/ — JS entry kept as scaffold data.
        // Dữ liệu scaffold vẫn có RN, nhưng runner MagiCore không được gọi npm.
        write_file(
            &target.join("react-native").join("package.json"),
            &format!(
                "{{\n  \"name\": \"{}\",\n  \"version\": \"0.1.0\",\n  \"private\": true,\n  \"scripts\": {{\n    \"android\": \"react-native run-android\",\n    \"ios\": \"react-native run-ios\"\n  }},\n  \"dependencies\": {{\n    \"react\": \"18.2.0\",\n    \"react-native\": \"0.74.0\"\n  }}\n}}\n",
                slug
            ),
        )?;
        write_file(
            &target.join("react-native").join("index.js"),
            "const { AppRegistry } = require('react-native');\nconst App = require('./App');\n\nAppRegistry.registerComponent('main', () => App);\n",
        )?;
        write_file(
            &target.join("react-native").join("App.js"),
            "const React = require('react');\nconst { Text } = require('react-native');\n\nmodule.exports = function App() {\n  return React.createElement(Text, null, 'hello from MagiCore (react-native)');\n};\n",
        )?;

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
}
