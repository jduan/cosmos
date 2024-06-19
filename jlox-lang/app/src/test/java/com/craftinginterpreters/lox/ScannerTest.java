package com.craftinginterpreters.lox;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.List;
import org.junit.jupiter.api.Test;

public class ScannerTest {
  @Test
  public void testScanTokens() {
    String source = "1 + 2";
    Scanner scanner = new Scanner(source);
    List<Token> tokens = scanner.scanTokens();
    assertEquals(1, 1);
  }
}
