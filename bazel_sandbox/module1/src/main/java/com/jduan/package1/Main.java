package com.jduan.package1;

import com.google.common.base.Joiner;
import com.jduan.package2.Foo;

public class Main {
  public static void main(String[] args) {
    System.out.println("Hello world!");
    System.out.println(Joiner.on(" ").join(new Foo().getFoos()));
  }
}
