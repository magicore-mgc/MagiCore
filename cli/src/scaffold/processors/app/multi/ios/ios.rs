//! iOS and Objective-C bridge output implementation.
//! Triển khai output iOS và cầu nối Objective-C.

use super::super::super::MultiProjectContext;
use anyhow::Result;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::app::implementation) fn write(
    context: &MultiProjectContext<'_>,
) -> Result<()> {
    let target = context.target;
    let slug = context.slug.as_str();

    // The Swift Package and native Objective-C bridge are emitted together.
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
            .join(slug)
            .join("main.swift"),
        "print(\"MagiCore iOS app scaffold (swift)\")\n",
    )?;
    // The Objective-C bridge stays outside SPM Sources because SPM does not mix these languages in one target.
    // ObjC bridge nằm ngoài SPM Sources (Xcode native project dùng) — SPM không chấp mixed language.
    write_file(
        &target.join("ios").join("ObjcBridge").join("ObjcBridge.h"),
        "#import <Foundation/Foundation.h>\n\n@interface MGShared : NSObject\n+ (NSString *)hello;\n@end\n",
    )?;
    write_file(
        &target.join("ios").join("ObjcBridge").join("ObjcBridge.m"),
        "#import \"ObjcBridge.h\"\n\n@implementation MGShared\n+ (NSString *)hello {\n    return @\"hello from MagiCore shared (objc)\";\n}\n@end\n",
    )?;
    Ok(())
}
