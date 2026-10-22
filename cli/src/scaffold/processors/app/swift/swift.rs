//! Swift Package implementation.
//! Triển khai Swift Package.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::app::implementation) fn write(
    target: &Path,
    name: &str,
) -> Result<()> {
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
    Ok(())
}
