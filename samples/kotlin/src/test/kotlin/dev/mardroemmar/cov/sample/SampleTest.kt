package dev.mardroemmar.cov.sample

import org.junit.jupiter.api.Disabled
import org.junit.jupiter.api.Test

class SampleTest {
  // This file won't have the most idiomatic code. It's intended to collect coverage and show off some kind of test someone might write.

  @Test
  fun testConstructor() {
    Sample()
  }

  @Test
  @Disabled("It's never supposed to be enabled.")
  fun testNeverCalled() {
    val sample = Sample()
    sample.neverCalled()
  }

  @Test
  fun testCallingOnce() {
    val sample = Sample()
    sample.calledOnce()
  }

  @Test
  fun testCallingManyTimes() {
    val sample = Sample()
    sample.calledManyTimes("Alice")
    sample.calledManyTimes("Bob")
  }

  @Test
  fun testLooped() {
    val sample = Sample()
    repeat(500) {
      sample.looped("Alice")
    }
  }
}
