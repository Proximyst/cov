package dev.mardroemmar.cov.sample;

public class Sample {

  public void neverCalled() {
    System.out.println("This method is never called.");
  }

  public void calledOnce() {
    System.out.println("This method is called exactly once.");
  }

  public void calledManyTimes(String name) {
    var greeting = "Hello, %s!".formatted(name);
    System.out.println("I have gotten a greeting to share. I'll print it on stderr.");
    System.err.println(greeting);
    System.out.println("Did you get that?");
  }
}
