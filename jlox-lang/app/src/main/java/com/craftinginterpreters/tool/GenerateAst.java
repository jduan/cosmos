package com.craftinginterpreters.tool;

import java.io.IOException;
import java.io.PrintWriter;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;

public class GenerateAst {
  public static void main(String[] args) throws IOException {
    if (args.length != 1) {
      System.err.println("Usage: generate_ast <output_directory>");
      System.exit(64);
    }

    String outputDir = args[0];
    defineAst(
        outputDir,
        "Expr",
        Arrays.asList(
            "Binary   : Expr left, Token operator, Expr right",
            "Grouping : Expr expression",
            "Literal  : Object value",
            "Unary    : Token operator, Expr right"));
  }

  private static void defineAst(String outputDir, String baseName, List<String> types)
      throws IOException {
    String path = String.format("%s/%s.java", outputDir, baseName);
    PrintWriter writer = new PrintWriter(path, "UTF-8");
    writer.println(
        """
        package com.craftinginterpreters.lox;

        import java.util.List;

        abstract class %s {
        """
            .formatted(baseName));
    defineVisitor(writer, baseName, types);
    for (String type : types) {
      String className = type.split(":")[0].trim();
      String fields = type.split(":")[1].trim();
      defineType(writer, baseName, className, fields);
    }

    // The base accept() method
    writer.println();
    writer.println("abstract <R> R accept(Visitor<R> visitor);");

    writer.println("}");
    writer.close();
  }

  private static void defineVisitor(PrintWriter writer, String baseName, List<String> types) {
    writer.println("interface Visitor<R> {");
    for (String type : types) {
      String typeName = type.split(":")[0].trim();
      writer.println(
          "R visit%s%s (%s %s);".formatted(typeName, baseName, typeName, baseName.toLowerCase()));
    }
    writer.println("}");
  }

  // Return the definition of an Expr subclass, such as Binary
  private static void defineType(
      PrintWriter writer, String baseName, String className, String fields) {
    writer.println("static class %s extends Expr {".formatted(className));
    List<String> names = new LinkedList<>();
    for (String field : fields.split(",")) {
      String type = field.trim().split(" ")[0].trim();
      String name = field.trim().split(" ")[1].trim();
      names.add(name);

      writer.println(String.format("final %s;", field));
    }
    writer.println();

    // constructor
    writer.println("%s(%s) {".formatted(className, fields));
    for (String name : names) {
      writer.println("this.%s = %s;".formatted(name, name));
    }
    writer.println("}");

    // Visitor pattern
    writer.println();
    writer.println(
        """
@Override
<R> R accept(Visitor<R> visitor) {
  return visitor.visit%s%s(this);
}
"""
            .formatted(className, baseName));

    writer.println("}");
  }
}
