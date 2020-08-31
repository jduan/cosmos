package jduan.pmd_rules

import net.sourceforge.pmd.lang.java.ast.ASTMethodDeclaration
import net.sourceforge.pmd.lang.java.ast.ASTName
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
