//! React Native implementation.
//! Triển khai React Native.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::{slugify, write_file};

pub(in crate::scaffold::processors::app::implementation) fn write(
    target: &Path,
    name: &str,
) -> Result<()> {
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
    Ok(())
}
