package com.jduan.package1;

import org.junit.Test;
import org.junit.runner.RunWith;
import org.junit.runners.JUnit4;

@RunWith(JUnit4.class)
public class MainTest {
  @Test
  public void test1() throws Exception {
    Main.main(new String[] {"Foo"});
  }
}
