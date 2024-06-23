package com.craftinginterpreters.lox;

// This is a visitor class that prints an AST in the "reverse polish notation" format.
// eg: (1 + 2) * (4 - 3)
// becomes: 1 2 + 4 3 - *
public class AstPrinterPRN implements Expr.Visitor<String> {

  @Override
  public String visitBinaryExpr(Expr.Binary expr) {
    return "%s %s %s"
        .formatted(expr.left.accept(this), expr.right.accept(this), expr.operator.lexeme);
  }

  @Override
  public String visitGroupingExpr(Expr.Grouping expr) {
    return "%s group".formatted(expr.expression.accept(this));
  }

  @Override
  public String visitLiteralExpr(Expr.Literal expr) {
    if (expr.value == null) return "nil";
    return expr.value.toString();
  }

  @Override
  public String visitUnaryExpr(Expr.Unary expr) {
    return "%s %s".formatted(expr.right.accept(this), expr.operator.lexeme);
  }

  // testing only
  public static void main(String[] args) {
    Expr expression =
        new Expr.Binary(
            new Expr.Unary(new Token(TokenType.MINUS, "-", null, 1), new Expr.Literal(123)),
            new Token(TokenType.STAR, "*", null, 1),
            new Expr.Grouping(new Expr.Literal(45.67)));
    System.out.println(expression.accept(new AstPrinterPRN()));
  }
}
