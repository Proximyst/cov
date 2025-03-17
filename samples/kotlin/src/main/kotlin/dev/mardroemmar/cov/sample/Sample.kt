package dev.mardroemmar.cov.sample

class Sample {
  fun neverCalled() {
    println("This method is never called.")
  }

  fun calledOnce() {
    println("This method is called once.")
  }

  // Weird formatting for the `method`'s `line` field.
  fun
  calledManyTimes(name: String) {
    // Non-executable code.

    val greeting = "Hello, $name!"
    println("I have gotten a greeting to share.")
    println(greeting)
  }

  fun looped(name: String) {
    val greeting = "Hello, $name!"
    println(greeting)
  }

  fun `has a fun snowman! ☃️ 💛`() {
    println("Wow! That was exciting, wasn't it?")
  }
}
