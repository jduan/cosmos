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

  @Test
  public void testSingleCharacterTokens() {
    String source = "() { } ,.-+ ; * ! != = == <= < >= >";
    Scanner scanner = new Scanner(source);
    List<Token> tokens = scanner.scanTokens();
    List<Token> expectedTokens =
        List.of(
            new Token(TokenType.LEFT_PAREN, "(", null, 1),
            new Token(TokenType.RIGHT_PAREN, ")", null, 1),
            new Token(TokenType.LEFT_BRACE, "{", null, 1),
            new Token(TokenType.RIGHT_BRACE, "}", null, 1),
            new Token(TokenType.COMMA, ",", null, 1),
            new Token(TokenType.DOT, ".", null, 1),
            new Token(TokenType.MINUS, "-", null, 1),
            new Token(TokenType.PLUS, "+", null, 1),
            new Token(TokenType.SEMICOLON, ";", null, 1),
            new Token(TokenType.STAR, "*", null, 1),
            new Token(TokenType.BANG, "!", null, 1),
            new Token(TokenType.BANG_EQUAL, "!=", null, 1),
            new Token(TokenType.EQUAL, "=", null, 1),
            new Token(TokenType.EQUAL_EQUAL, "==", null, 1),
            new Token(TokenType.LESS_EQUAL, "<=", null, 1),
            new Token(TokenType.LESS, "<", null, 1),
            new Token(TokenType.GREATER_EQUAL, ">=", null, 1),
            new Token(TokenType.GREATER, ">", null, 1),
            new Token(TokenType.EOF, "", null, 1));
    assertEquals(expectedTokens, tokens);
  }

  @Test
  public void testComments() {
    String source =
        """
1 + 2 // this is a comment
3 / 4
""";
    Scanner scanner = new Scanner(source);
    List<Token> tokens = scanner.scanTokens();
    List<Token> expectedTokens =
        List.of(
            new Token(TokenType.NUMBER, "1", Double.valueOf(1.0), 1),
            new Token(TokenType.PLUS, "+", null, 1),
            new Token(TokenType.NUMBER, "2", Double.valueOf(2.0), 1),
            new Token(TokenType.NUMBER, "3", Double.valueOf(3.0), 2),
            new Token(TokenType.SLASH, "/", null, 2),
            new Token(TokenType.NUMBER, "4", Double.valueOf(4.0), 2),
            new Token(TokenType.EOF, "", null, 3));
    assertEquals(expectedTokens, tokens);
  }

  @Test
  public void testWhitespace() {
    String source =
        """
1 +    2 // this is a comment



3 /       4
""";
    Scanner scanner = new Scanner(source);
    List<Token> tokens = scanner.scanTokens();
    List<Token> expectedTokens =
        List.of(
            new Token(TokenType.NUMBER, "1", Double.valueOf(1.0), 1),
            new Token(TokenType.PLUS, "+", null, 1),
            new Token(TokenType.NUMBER, "2", Double.valueOf(2.0), 1),
            new Token(TokenType.NUMBER, "3", Double.valueOf(3.0), 5),
            new Token(TokenType.SLASH, "/", null, 5),
            new Token(TokenType.NUMBER, "4", Double.valueOf(4.0), 5),
            new Token(TokenType.EOF, "", null, 6));
    assertEquals(expectedTokens, tokens);
  }

  @Test
  public void testStrings() {
    String source =
        """
1 +    "hello 1 3" // this is a comment



3 /       4
""";
    Scanner scanner = new Scanner(source);
    List<Token> tokens = scanner.scanTokens();
    List<Token> expectedTokens =
        List.of(
            new Token(TokenType.NUMBER, "1", Double.valueOf(1.0), 1),
            new Token(TokenType.PLUS, "+", null, 1),
            new Token(TokenType.STRING, "\"hello 1 3\"", "hello 1 3", 1),
            new Token(TokenType.NUMBER, "3", Double.valueOf(3.0), 5),
            new Token(TokenType.SLASH, "/", null, 5),
            new Token(TokenType.NUMBER, "4", Double.valueOf(4.0), 5),
            new Token(TokenType.EOF, "", null, 6));
    assertEquals(expectedTokens, tokens);
  }

  @Test
  public void testDigits() {
    String source =
        """
1 +    "hello 1 3" // this is a comment



3.14 /       4.5
""";
    Scanner scanner = new Scanner(source);
    List<Token> tokens = scanner.scanTokens();
    List<Token> expectedTokens =
        List.of(
            new Token(TokenType.NUMBER, "1", Double.valueOf(1.0), 1),
            new Token(TokenType.PLUS, "+", null, 1),
            new Token(TokenType.STRING, "\"hello 1 3\"", "hello 1 3", 1),
            new Token(TokenType.NUMBER, "3.14", Double.valueOf(3.14), 5),
            new Token(TokenType.SLASH, "/", null, 5),
            new Token(TokenType.NUMBER, "4.5", Double.valueOf(4.5), 5),
            new Token(TokenType.EOF, "", null, 6));
    assertEquals(expectedTokens, tokens);
  }

  @Test
  public void testIdentifiers() {
    String source =
        """
1 +    "hello 1 3" // this is a comment



3.14 /       4.5
amount + LENGTH + _width
""";
    Scanner scanner = new Scanner(source);
    List<Token> tokens = scanner.scanTokens();
    List<Token> expectedTokens =
        List.of(
            new Token(TokenType.NUMBER, "1", Double.valueOf(1.0), 1),
            new Token(TokenType.PLUS, "+", null, 1),
            new Token(TokenType.STRING, "\"hello 1 3\"", "hello 1 3", 1),
            new Token(TokenType.NUMBER, "3.14", Double.valueOf(3.14), 5),
            new Token(TokenType.SLASH, "/", null, 5),
            new Token(TokenType.NUMBER, "4.5", Double.valueOf(4.5), 5),
            new Token(TokenType.IDENTIFIER, "amount", null, 6),
            new Token(TokenType.PLUS, "+", null, 6),
            new Token(TokenType.IDENTIFIER, "LENGTH", null, 6),
            new Token(TokenType.PLUS, "+", null, 6),
            new Token(TokenType.IDENTIFIER, "_width", null, 6),
            new Token(TokenType.EOF, "", null, 7));
    assertEquals(expectedTokens, tokens);
  }
}
