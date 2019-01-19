package jduan.conventions

import java.lang.IndexOutOfBoundsException

data class Point(val x: Int, val y: Int) {
  operator fun plus(other: Point): Point =
      Point(x + other.x, y + other.y)
}

operator fun Point.minus(other: Point) =
    Point(x - other.x, y - other.y)

// When you define an operator, you don’t need to use the same types for the two operands.
operator fun Point.times(scale: Double) =
    Point((x * scale).toInt(), (y * scale).toInt())

// Note that Kotlin operators don’t automatically support commutativity (the ability to swap
// the left and right sides of an operator).
operator fun Double.times(p: Point) =
    Point((this * p.x).toInt(), (this * p.y).toInt())

// The return type of an operator function can also be different from either of the operand types.
operator fun Char.times(count: Int): String {
  return toString().repeat(count)
}

operator fun Point.unaryMinus(): Point = Point(-x, -y)

operator fun Point.get(index: Int): Int {
  return when (index) {
    0 -> x
    1 -> y
    else -> throw IndexOutOfBoundsException("Invalid coordinate $index")
  }
}

data class MutablePoint(var x: Int, var y: Int)

operator fun MutablePoint.set(index: Int, value: Int) {
  when (index) {
    0 -> x = value
    1 -> y = value
    else -> throw IndexOutOfBoundsException("Invalid coordinate $index")
  }
}

fun runConventions() {
  val p1 = Point(10, 20)
  val p2 = Point(30, 40)
  println(p1 + p2)
  println(p1 - p2)
  println(p1 * 1.5)
  println(1.5 * p1)
  println('a' * 3)
  println(-p1)
  println(p1[0])
  val p3 = MutablePoint(10, 20)
  p3[1] = 42
  println(p3)
}
