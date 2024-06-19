package com.craftinginterpreters.lox;

// import java.util.Scanner;
import java.util.Scanner;
import org.junit.jupiter.api.Test;

public class ScannerTest {
  @Test
  public void testScanTokens() {
    String source = "1 + 2";
    Scanner scanner = new Scanner(source);
  }
}
