package year2018.day1

import kotlin.test.assertEquals
import org.junit.Test

class Day1Test {
  @Test fun test1() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2018/Day1Input.txt"
    assertEquals(510, calculateFrequency(inputFile))
  }

  @Test fun test2() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2018/Day1Input.txt"
    assertEquals(69074, firstRepeatedFrequency(inputFile))
  }
}
