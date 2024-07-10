package com.craftinginterpreters.lox;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class AstPrinterTest {
  @Test
  public void testAstPrinter() {
    Expr binaryExpr =
        new Expr.Binary(
            new Expr.Unary(new Token(TokenType.MINUS, "-", null, 1), new Expr.Literal(123)),
            new Token(TokenType.STAR, "*", null, 1),
            new Expr.Grouping(
                new Expr.Binary(
                    new Expr.Literal(45.67),
                    new Token(TokenType.PLUS, "+", null, 1),
                    new Expr.Literal(null))));
    Expr expression =
        new Expr.Binary(binaryExpr, new Token(TokenType.MINUS, "-", null, 1), binaryExpr);

    assertEquals(
        "(- (* (- 123) (group (+ 45.67 nil))) (* (- 123) (group (+ 45.67 nil))))",
        expression.accept(new AstPrinter()));
  }
}
