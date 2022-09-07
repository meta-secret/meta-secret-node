import XCTest
@testable import testLib

final class testLibTests: XCTestCase {
    func testExample() throws {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct
        // results.
        XCTAssertEqual(testLib().text, "Hello, World!")
    }
}
