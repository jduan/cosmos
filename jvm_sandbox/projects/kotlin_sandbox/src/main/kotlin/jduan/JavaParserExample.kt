package jduan

import com.github.javaparser.StaticJavaParser
import com.github.javaparser.ast.body.MethodDeclaration
import com.github.javaparser.ast.body.Parameter
import com.github.javaparser.ast.expr.BinaryExpr
import com.github.javaparser.ast.expr.Expression
import java.util.stream.Collectors


object ParsingForFirstTime {
    @JvmStatic
    fun main(args: Array<String>) {
        val javaCode = """
import com.airbnb.common.metrics.TaggedMetricRegistryFactory;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.assertEquals;

public class MyTests {
    @Before
    public void setUp() {
      injector = Guice.createInjector(new AopMetricsModule());
      testEndpoints = injector.getInstance(TestEndpoints.class);
    }

    @Before
    public void setUpMetricRegistry() {
      TaggedMetricRegistryFactory.initialize(taggedMetricRegistry);
    }

    @After
    public void tearDownMetricRegistry() {
      TaggedMetricRegistryFactory.initialize(new TaggedMetricRegistry());
    }

    @Test
    public void multiplicationOfZeroIntegersShouldReturnZero() {
        MyClass tester = new MyClass(); // MyClass is tested

        // assert statements
        assertEquals(0, tester.multiply(10, 0), "10 x 0 must be 0");
        assertEquals(0, tester.multiply(0, 10), "0 x 10 must be 0");
        assertEquals(0, tester.multiply(0, 0), "0 x 0 must be 0");
    }
}
        """.trimIndent()

        // Parse an expression
        val expressionNode = StaticJavaParser.parseExpression<Expression>("1 + 2")

        // Parse a body declaration: it could be either a field or a method or an inner class
        val methodNode = StaticJavaParser.parseBodyDeclaration(
            "boolean invert(boolean aFlag) { return !p; }")

        // If we know the expression is a binary expression we can cast it and access more
        // specific information like the element on the left and on the right and the operator
        val binaryExpr = expressionNode as BinaryExpr
        println(String.format("Binary expression: left=%s, right=%s, operator=%s",
            binaryExpr.left, binaryExpr.right, binaryExpr.operator))

        // Here we know we have a method declaration. We may want to figure out specific
        // things like the name or the return type of the method.
        // We transform the parameters to get only the names: we are not interested in
        // in printing the whole nodes corresponding to the parameters
        val methodDeclaration = methodNode as MethodDeclaration
        println(String.format("Method declaration: modifiers=%s, name=%s, parameters=%s, returnType=%s",
            methodDeclaration.modifiers, methodDeclaration.name,
            methodDeclaration.parameters.stream().map { p: Parameter -> p.name }.collect(Collectors.toList()),
            methodDeclaration.type))

        // Parse the code of an entire source file, a.k.a. a Compilation Unit
        val compilationUnitNode = StaticJavaParser.parse(javaCode)

        // We can navigate the compilation unit to extract a single class. In this case
        // the CompilationUnit contains only this class but in general it could contains
        // a package declaration, imports and several type declarations
        val classDeclaration = compilationUnitNode.getClassByName("MyTests").get()
        println(String.format("Class declaration: name=%s, nMembers=%s",
            classDeclaration.name, classDeclaration.members.size))
        classDeclaration.members.forEach {
            it.annotations.forEach {
                println("annotation: $it")
            }
        }

        // Check if a class is imported
        val hasImport = compilationUnitNode.imports.any { import ->
            import.nameAsString.contains("com.airbnb.common.metrics.TaggedMetricRegistryFactory")
        }
        println("hasImport: $hasImport")

        // Find all classes
        val classes = compilationUnitNode.types
            .filter { it.isClassOrInterfaceDeclaration }
            .map { it.toClassOrInterfaceDeclaration().get() }

        // Find methods that have @Before annotations
        classes.forEach { klass ->
            val methods = klass.members.filter {
                it.annotations.any {
                    it.nameAsString == "Before" || it.nameAsString == "BeforeClass"
                }
            }

            // find if methods have a statement that calls "TaggedMetricRegistryFactory.initialize"
            methods.forEach {
                if (it.isMethodDeclaration) {
                    val method = it.asMethodDeclaration();
                    if (method.body.isPresent) {
                        val body = method.body.get()
                        body.statements.forEach { stmt ->
                            if (stmt.toString().contains("TaggedMetricRegistryFactory.initialize")) {
                                println("stmt: $stmt")
                            }
                        }
                    }
                }
            }
        }
    }
}
