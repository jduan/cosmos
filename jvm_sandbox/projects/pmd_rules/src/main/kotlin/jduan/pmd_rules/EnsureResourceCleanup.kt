package jduan.pmd_rules

import net.sourceforge.pmd.lang.java.ast.ASTMethodDeclaration
import net.sourceforge.pmd.lang.java.ast.ASTPrimaryExpression
import net.sourceforge.pmd.lang.java.rule.AbstractJavaRule

class EnsureResourceCleanup : AbstractJavaRule() {
    override fun visit(node: ASTMethodDeclaration, data: Any): Any {
        println("annotations: ${node.declaredAnnotations}")

        val exps = node.findDescendantsOfType(ASTPrimaryExpression::class.java)

        exps.forEach {
            println("expression: $it")
        }

        super.visit(node, data)
        return data
    }
}
