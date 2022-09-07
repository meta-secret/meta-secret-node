public class Greeting {

  var greeting = "Hello"

  init() {}

  public init(greeting: String) {
    self.greeting = greeting
  }

  public func outputGreeting() {
    print(greeting)
  }

}