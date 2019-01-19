package year2018.day5

import java.util.*

fun reactor(polymer: String): String {
  val stack = Stack<Char>()

  polymer.forEach { char ->
    if (stack.empty()) {
      stack.push(char)
    } else {
      val top = stack.peek()
      val diff = top - char
      if (diff == 32 || diff == -32) {
        stack.pop()
      } else {
        stack.push(char)
      }
    }
  }

  return stack.toList().joinToString(separator = "")
}

fun part2(polymer: String): Int {
  var min: Int = Integer.MAX_VALUE
  for (ch in 'a' .. 'z') {
    val newPolymer = polymer.filterNot { char -> char.toLowerCase() == ch }
    val remains = reactor(newPolymer)
    if (remains.length < min)
      min = remains.length
    println("after removing ${ch}, remains size: ${remains.length}")
  }

  return min
}
