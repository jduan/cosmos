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
    List<Token> expectedTokens =
        List.of(
            new Token(TokenType.NUMBER, "1", Double.valueOf(1.0), 1),
            new Token(TokenType.PLUS, "+", null, 1),
            new Token(TokenType.NUMBER, "2", Double.valueOf(2.0), 1),
            new Token(TokenType.EOF, "", null, 1));
    assertEquals(expectedTokens, tokens);
  }
}
