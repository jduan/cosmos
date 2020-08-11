package year2018.day1

import java.io.File

fun calculateFrequency(inputFile: String): Long {
  var frequency = 0L
  File(inputFile).forEachLine {
    frequency += it.toInt()
  }
  return frequency
}

fun firstRepeatedFrequency(inputFile: String): Long {
  var frequency = 0L
  val frequencies = mutableSetOf<Long>(frequency)
  loop@ while (true) {
    val reader = File(inputFile).bufferedReader()
    var line: String?
    while (true) {
      line = reader.readLine()
      if (line == null) break

      frequency += line.toInt()
      if (frequencies.contains(frequency)) {
        break@loop
      } else {
        frequencies.add(frequency)
      }
    }
  }
  return frequency
}
