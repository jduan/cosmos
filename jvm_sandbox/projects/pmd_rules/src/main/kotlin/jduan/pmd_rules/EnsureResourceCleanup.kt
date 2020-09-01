package jduan.pmd_rules

import net.sourceforge.pmd.lang.java.ast.ASTAllocationExpression
import net.sourceforge.pmd.lang.java.ast.ASTClassOrInterfaceType
import net.sourceforge.pmd.lang.java.ast.ASTMethodDeclaration
import net.sourceforge.pmd.lang.java.ast.ASTName
import net.sourceforge.pmd.lang.java.ast.ASTPrimaryExpression
import net.sourceforge.pmd.lang.java.ast.ASTPrimaryPrefix
import net.sourceforge.pmd.lang.java.rule.AbstractJavaRule

/**
 * This PMD check looks for a matching pair of "System.currentTimeMillis()" call in
 * @Before and @After methods for junit tests. This can be generalized to check for
 * any kind of resource initialization and cleanup.
 */
class EnsureResourceCleanup : AbstractJavaRule() {
    var currentTimeMillisCalled = false

    override fun visit(node: ASTMethodDeclaration, data: Any): Any {
        if (isBefore(node)) {
            println("isGetInstanceCalled: ${isGetInstanceCalled(node)}")
            currentTimeMillisCalled = isCurrentTimeMillisCalled(node)
        } else if (isAfter(node)){
            if (currentTimeMillisCalled) {
                if (hasViolation(node)) {
                    addViolationWithMessage(data, node,
                        "System.currentTimeMillis is called in @Before but not in @After")
                }
            }
        }

        super.visit(node, data)
        return data
    }


    /**
     * Look for a call like this: Order.getInstance(new String("hello"))
     */
    private fun isGetInstanceCalled(node: ASTMethodDeclaration): Boolean {
        val exps = node.findDescendantsOfType(ASTPrimaryExpression::class.java)

        val primaryExpression = exps.firstOrNull {
            val prefix = it.getFirstDescendantOfType(ASTPrimaryPrefix::class.java)
            val name = prefix.getFirstDescendantOfType(ASTName::class.java)
            if (name != null) {
                name.image == "Order.getInstance"
            } else {
                false
            }
        }

        if (primaryExpression != null) {
            val allocation = primaryExpression.getFirstDescendantOfType(ASTAllocationExpression::class.java)
            if (allocation != null) {
                val classType = allocation.getFirstDescendantOfType(ASTClassOrInterfaceType::class.java)
                println("classType: ${classType.image}")
                return classType.image == "String"
            }
        }

        return false
    }

    private fun isCurrentTimeMillisCalled(node: ASTMethodDeclaration): Boolean {
        val exps = node.findDescendantsOfType(ASTPrimaryPrefix::class.java)

        return exps.any {
            val name = it.getFirstDescendantOfType(ASTName::class.java)
            if (name != null) {
                name.image == "System.currentTimeMillis"
            } else {
                false
            }
        }
    }

    private fun hasViolation(node: ASTMethodDeclaration): Boolean {
        return !isCurrentTimeMillisCalled(node)
    }

    private fun isBefore(node: ASTMethodDeclaration) = hasAnnotation(node, "Before")

    private fun isAfter(node: ASTMethodDeclaration) = hasAnnotation(node, "After")

    private fun hasAnnotation(node: ASTMethodDeclaration, annotationName: String): Boolean {
        return node.declaredAnnotations.any {
            val name = it.getFirstDescendantOfType(ASTName::class.java)
            name?.image == annotationName
        }
    }
}
