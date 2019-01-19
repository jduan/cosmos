package year2018.day2

import java.io.File
import java.lang.Exception
import java.util.*

fun checksum(path: String): Int {
  var totalTwos = 0
  var totalThrees = 0
  File(path).forEachLine {
    val (twos, threes) = countID2(it)
    totalTwos += twos
    totalThrees += threes
  }

  return (totalTwos * totalThrees)
}

// Another way of counting twos and threes.
fun countID2(id: String): Pair<Int, Int> {
  val counts = mutableMapOf<Char, Int>()
  id.forEach {
    if (counts.contains(it)) {
      counts[it] = counts[it]!! + 1
    } else {
      counts[it] = 1
    }
  }
  val twos = if (counts.containsValue(2)) 1 else 0
  val threes = if (counts.containsValue(3)) 1 else 0
  return Pair(twos, threes)
}

/**
 * Given a string ID, check if it contains exactly 2 of any letter and
 * separately check if it contains exactly 3 of any letter.
 * Return a pair of both. Note that if there are multiple letters that
 * appear exactly twice, it only counts once.
 */
fun countID(id: String): Pair<Int, Int> {
  val sortedId = sortString(id)
  var twos = 0
  var threes = 0
  var previous: Char? = null
  var i = 0
  var repeats = 0
  while (i < sortedId.length) {
    val current = sortedId[i]
    i++
    if (previous == null) {
      previous = current
      repeats = 1
      continue
    }
    if (previous == current) {
      repeats++
    } else {
      if (repeats >= 3) {
        threes = 1
      } else if (repeats == 2) {
        twos = 1
      }
      previous = current
      repeats = 1
    }
  }
  // Don't forget about the last batch!
  if (repeats >= 3) {
    threes = 1
  } else if (repeats == 2) {
    twos = 1
  }
  return Pair(twos, threes)
}

fun findCommonLetters(path: String): String {
  val lines = File(path).readLines()
  for (i in 0 until lines.size) {
    for (j in i until lines.size) {
      if (distance(lines[i], lines[j]) == 1) {
        return common(lines[i], lines[j])
      }
    }
  }

  throw Exception("Not found!")
}

internal fun sortString(s: String): String {
  val chars = s.toCharArray()
  Arrays.sort(chars)
  return String(chars)
}

// Return the distance of 2 strings of the same length.
fun distance(s1: String, s2: String): Int {
  var diff = 0
  for (i in 0 until s1.length) {
    diff += if (s1[i] == s2[i]) 0 else 1
  }
  return diff
}

// Return the common letters between 2 strings of the same length.
fun common(s1: String, s2: String): String {
  val letters = mutableListOf<Char>()
  for (i in 0 until s1.length) {
    if (s1[i] == s2[i]) {
      letters.add(s1[i])
    }
  }
  return letters.joinToString(separator = "")
}
