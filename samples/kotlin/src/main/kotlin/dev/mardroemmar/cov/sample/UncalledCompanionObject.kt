package dev.mardroemmar.cov.sample

class UncalledCompanionObject {
  companion object {
    fun neverCalled() {
      println("This entire class is never called.")
    }
  }
}
