package com.jduan.package2;

import java.util.List;
import org.junit.Assert;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.junit.runners.JUnit4;

@RunWith(JUnit4.class)
public class BarTest {
  @Test
  public void test3() throws Exception {
    Foo foo = new Foo();
    List<String> foos = foo.getFoos();
    Assert.assertFalse(!foos.isEmpty());
  }
}
