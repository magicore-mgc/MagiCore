//! React Native output implementation.
//! Triển khai output React Native.

use super::super::super::MultiProjectContext;
use anyhow::Result;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::app::implementation) fn write(
    context: &MultiProjectContext<'_>,
) -> Result<()> {
    let target = context.target;
    let slug = context.slug.as_str();

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
    Ok(())
}
