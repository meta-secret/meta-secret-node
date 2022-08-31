// swift-tools-version:5.3
import PackageDescription
import Foundation
let package = Package(
        name: "MetaSecretCoreLib",
        platforms: [
            .iOS(.v13),
        ],
        products: [
            .library(
                name: "MetaSecretCoreLib",
                targets: ["MetaSecretCoreLib"]),
        ],
        targets: [
            .target(
                name: "MetaSecretCoreLib",
                dependencies: ["MetaSecretCore"]),
            .binaryTarget(
                name: "MetaSecretCore",
                url: "https://github.com/...../bundle.zip",
                checksum: "ea7a.....35b2"),
            .testTarget(
                name: "RustToSwiftTests",
                dependencies: ["RustToSwift"]),
        ]
)