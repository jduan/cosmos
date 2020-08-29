package jduan.pmd_rules

import net.sourceforge.pmd.lang.java.ast.ASTVariableDeclaratorId
import net.sourceforge.pmd.lang.java.rule.AbstractJavaRule

class DetectOffensiveVariableNames : AbstractJavaRule() {
    override fun visit(node: ASTVariableDeclaratorId, data: Any): Any {
        if (node.name == "wtf") {
            addViolation(data, node)
        }

        super.visit(node, data)
        return data
    }
}
