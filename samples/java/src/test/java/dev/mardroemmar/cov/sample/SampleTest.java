package dev.mardroemmar.cov.sample;

import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

class SampleTest {
  // This file won't have the most idiomatic code. It's intended to collect coverage and show off some kind of test someone might write.

  @Test
  void testConstructor() {
    new Sample();
  }

  @Test
  @Disabled("It's never supposed to be enabled.")
  void testNeverCalled() {
    Sample sample = new Sample();
    sample.neverCalled();
  }

  @Test
  void testCallingOnce() {
    Sample sample = new Sample();
    sample.calledOnce();
  }

  @Test
  void testCallingManyTimes() {
    Sample sample = new Sample();
    sample.calledManyTimes("Alice");
    sample.calledManyTimes("Bob");
  }

  @Test
  void testLooped() {
    Sample sample = new Sample();
    for (int i = 0; i < 500; ++i) {
      sample.looped("Alice");
    }
  }
}
