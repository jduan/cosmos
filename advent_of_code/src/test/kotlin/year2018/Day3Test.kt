package year2018.day3

import kotlin.test.assertEquals
import org.junit.Test

class Day3Test {
  @Test fun test1() {
    assertEquals(Rectangle(16, 859, 624, 21, 19), parseLine("#16 @ 859,624: 21x19"))
    assertEquals(Rectangle(155, 754, 921, 21, 15), parseLine("#155 @ 754,921: 21x15"))
  }

  @Test fun test2() {
    val expected = listOf(
        Pair(0, 0),
        Pair(0, 1),
        Pair(1, 0),
        Pair(1, 1),
        Pair(2, 0),
        Pair(2, 1)
    )
    assertEquals(expected, Rectangle(1, 0, 0, 3, 2).coveredCells())
  }

  @Test fun test3() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2018/Day3Input.txt"
    assertEquals(98005, findSquareInches(inputFile))
  }

  @Test fun test4() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2018/Day3Input.txt"
    assertEquals(331, findSquareId(inputFile))
  }
}
