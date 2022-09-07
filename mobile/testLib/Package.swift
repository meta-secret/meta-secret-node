// swift-tools-version: 5.6
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "testLib",
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "testLib",
            targets: ["testLib"]),
    ],
    dependencies: [
        // Dependencies declare other packages that this package depends on.
        //.package(path: ),
    ],
    targets: [
        .systemLibrary(name: "meta_secret_core"),

        .target(
            name: "testLib",
            dependencies: ["meta_secret_core"]),

        .testTarget(
            name: "testLibTests",
            dependencies: ["testLib"]),
    ]
)
