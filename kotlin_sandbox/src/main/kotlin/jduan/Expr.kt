package jduan.expr

import java.lang.IllegalArgumentException

interface Expr

class Num(val value: Int) : Expr

class Sum(val left: Expr, val right: Expr) : Expr

fun eval(expr : Expr) : Int =
        if (expr is Num) {
            // smart cast: the compiler knows the type of expr is Num here
            // you can also perform an explicit cast if you want to:
            // val n = expr as Num
            expr.value
        } else if (expr is Sum) {
            eval(expr.left) + eval(expr.right)
        } else {
            throw IllegalArgumentException("Unknown expression!")
        }

fun eval2(expr : Expr) : Int =
        when (expr) {
            is Num -> expr.value
            is Sum -> eval(expr.left) + eval(expr.right)
            else -> throw IllegalArgumentException("Unknown expression!")
        }

fun evalWithLogging(expr : Expr) : Int =
        when (expr) {
            is Num -> {
                println("expr is a number")
                expr.value
            }
            is Sum -> {
                println("expr is a Sum")
                eval(expr.left) + eval(expr.right)
            }
            else -> throw IllegalArgumentException("Unknown expression!")
        }
