// swift-tools-version: 5.6
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "test-app",
    dependencies: [
        // Dependencies declare other packages that this package depends on.
        // .package(url: /* package url */, from: "1.0.0"),
        .package(path: "../testLib/")
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .executableTarget(
            name: "test-app",
            dependencies: [
                .byName(name: "testLib")
            ]),
        .testTarget(
            name: "test-appTests",
            dependencies: ["test-app"]),
    ]
)


/*
let package = Package(
    name: "DeckOfPlayingCards",
    products: [
        .library(name: "DeckOfPlayingCards", targets: ["DeckOfPlayingCards"]),
    ],
    dependencies: [
        .package(name: "PlayingCard",
                 url: "https://github.com/apple/example-package-playingcard.git",
                 from: "3.0.0"),
    ],
    targets: [
        .target(
            name: "DeckOfPlayingCards",
            dependencies: [
                .byName(name: "PlayingCard")
            ]),
        .testTarget(
            name: "DeckOfPlayingCardsTests",
            dependencies: [
                .target(name: "DeckOfPlayingCards")
            ]),
    ]
)
*/